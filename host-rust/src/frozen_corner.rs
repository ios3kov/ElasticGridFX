//! UI-only tentative coordinates. Render callbacks and saved streams never read
//! this state. No host pointer, frame, cache receipt, or parameter handle survives.
use super::*;
use std::cell::Cell;
use corner_ownership::Scope;
type Point=(i32,i32);
type Points=[Point;4];
type PreviewPoint=(usize,(f64,f64));

#[derive(Clone,Copy)]
struct Pending{scope:Scope,index:usize,original:Points,target:Point}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub(crate) enum Step{Cancelled,Preview,Idle,Unchanged,Commit{index:usize,target:Point}}
#[derive(Clone,Copy,Default)]
struct Transaction{pending:Option<Pending>}
impl Transaction{
    fn begin(&mut self,scope:Option<Scope>,index:usize,original:Points)->bool {
        self.pending=scope.filter(|_|index<4).map(|scope|Pending{scope,index,original,target:original[index]});
        self.pending.is_some()
    }
    fn preview(self,scope:Option<Scope>,current:Points)->Option<(usize,Point)> {
        self.pending.filter(|p|Some(p.scope)==scope&&p.original==current).map(|p|(p.index,p.target))
    }
    fn step(&mut self,scope:Option<Scope>,index:usize,current:Points,target:Point,last:bool)->Step {
        let Some(mut p)=self.pending.take() else{return Step::Cancelled;};
        if Some(p.scope)!=scope||p.index!=index||p.original!=current{return Step::Cancelled;}
        let changed=p.target!=target;p.target=target;
        if !last {self.pending=Some(p);return if changed{Step::Preview}else{Step::Idle};}
        if p.original[index]==target {Step::Unchanged}else{Step::Commit{index,target}}
    }
}
thread_local!{static TRANSACTION:Cell<Transaction>=const{Cell::new(Transaction{pending:None})};}

fn points(params:&ae::Parameters<Params>)->Result<Points,ae::Error>{
    let mut points=[(0,0);4];
    for (i,id) in plane::CORNERS.into_iter().enumerate(){
        let p=params.get(id)?;p.as_point()?;
        // Validate the union tag first and retain exact signed SDK16.16 bits.
        let raw=p.as_ref();points[i]=unsafe{(raw.u.td.x_value,raw.u.td.y_value)};
    }
    Ok(points)
}
pub(crate) fn clear(){TRANSACTION.set(Transaction::default());}
pub(crate) fn begin(input:&ae::InData,params:&ae::Parameters<Params>,event:&ae::EventExtra,
    id:Option<ae::aegp::PluginId>,index:usize)->Result<bool,ae::Error>{
    clear();
    let original=points(params)?;let mut tx=Transaction::default();
    let started=tx.begin(corner_ownership::scope(input,params,event,id),index,original);
    TRANSACTION.set(tx);Ok(started)
}
pub(crate) fn preview(input:&ae::InData,params:&ae::Parameters<Params>,event:&ae::EventExtra,
    id:Option<ae::aegp::PluginId>)->Result<Option<PreviewPoint>,ae::Error>{
    if TRANSACTION.get().pending.is_none(){return Ok(None);}
    Ok(TRANSACTION.get().preview(corner_ownership::scope(input,params,event,id),points(params)?)
        .map(|(i,(x,y))|(i,(f64::from(x)/65536.0,f64::from(y)/65536.0))))
}
pub(crate) fn step(input:&ae::InData,params:&ae::Parameters<Params>,event:&ae::EventExtra,
    id:Option<ae::aegp::PluginId>,index:usize,target:(f32,f32))->Result<Step,ae::Error>{
    if !target.0.is_finite()||!target.1.is_finite()||target.0.abs()>32767.0||target.1.abs()>32767.0 {
        clear();return Ok(Step::Cancelled);
    }
    let current=points(params)?;let mut tx=TRANSACTION.get();
    let action=tx.step(corner_ownership::scope(input,params,event,id),index,current,
        (ae::Fixed::from(target.0).as_fixed(),ae::Fixed::from(target.1).as_fixed()),event.last_time());
    TRANSACTION.set(tx);Ok(action)
}

#[cfg(test)]mod tests{
    use super::*;
    fn key()->Scope{corner_ownership::test_scope(7,10,2)}
    fn points()->Points{[(3,-7),(320<<16,0),(320<<16,240<<16),(0,240<<16)]}
    #[test]fn thousands_of_moves_keep_saved_points_unchanged_and_release_commits_once(){
        let original=points();let mut tx=Transaction::default();assert!(tx.begin(Some(key()),0,original));
        for x in 0..1000{assert_eq!(tx.step(Some(key()),0,original,(x,44),false),Step::Preview);
            assert_eq!(tx.preview(Some(key()),original),Some((0,(x,44))));}
        assert_eq!(tx.step(Some(key()),0,original,(999,44),true),Step::Commit{index:0,target:(999,44)});
        assert_eq!(tx.preview(Some(key()),original),None);
        assert_eq!(tx.step(Some(key()),0,original,(999,44),true),Step::Cancelled);
    }
    #[test]fn click_without_motion_or_return_to_exact_low_bits_creates_no_saved_edit(){
        let original=points();let mut tx=Transaction::default();tx.begin(Some(key()),0,original);
        assert_eq!(tx.step(Some(key()),0,original,original[0],true),Step::Unchanged);
        tx.begin(Some(key()),0,original);tx.step(Some(key()),0,original,(500,0),false);
        assert_eq!(tx.step(Some(key()),0,original,original[0],true),Step::Unchanged);
        assert_eq!(original[0],(3,-7));
    }
    #[test]fn duplicate_motion_is_idle_but_release_always_uses_latest_target(){
        let original=points();let mut tx=Transaction::default();
        tx.begin(Some(key()),0,original);
        for _ in 0..1000 {assert_eq!(tx.step(Some(key()),0,original,original[0],false),Step::Idle);}
        assert_eq!(tx.step(Some(key()),0,original,(500,44),false),Step::Preview);
        for _ in 0..1000 {assert_eq!(tx.step(Some(key()),0,original,(500,44),false),Step::Idle);}
        assert_eq!(tx.preview(Some(key()),original),Some((0,(500,44))));
        assert_eq!(tx.step(Some(key()),0,original,(500,44),true),Step::Commit{index:0,target:(500,44)});
        tx.begin(Some(key()),0,original);
        assert_eq!(tx.step(Some(key()),0,original,original[0],true),Step::Unchanged);
    }
    #[test]fn external_point_edit_and_changed_owner_time_mode_cancel_without_commit(){
        for other in [None,Some(corner_ownership::test_scope(8,10,2)),
            Some(corner_ownership::test_scope(7,11,2)),Some(corner_ownership::test_scope(7,10,4))]{
            let mut tx=Transaction::default();tx.begin(Some(key()),0,points());
            assert_eq!(tx.preview(other,points()),None);
            assert_eq!(tx.step(other,0,points(),(999,0),true),Step::Cancelled);
            assert!(tx.pending.is_none());
        }
        for i in 0..4 {let mut changed=points();changed[i].0+=1;
            let mut tx=Transaction::default();tx.begin(Some(key()),0,points());
            assert_eq!(tx.preview(Some(key()),changed),None);
            assert_eq!(tx.step(Some(key()),0,changed,(999,0),true),Step::Cancelled);}
    }
    #[test]fn independent_corners_repress_and_invalid_indices_do_not_inherit_tentative_state(){
        for i in 0..4 {let mut tx=Transaction::default();tx.begin(Some(key()),i,points());
            assert_eq!(tx.step(Some(key()),(i+1)%4,points(),(99,0),true),Step::Cancelled);
            assert!(tx.begin(Some(key()),i,points()));
            assert_eq!(tx.preview(Some(key()),points()),Some((i,points()[i])));}
        let mut tx=Transaction::default();assert!(!tx.begin(Some(key()),4,points()));
        assert!(!tx.begin(None,0,points()));
    }
}
