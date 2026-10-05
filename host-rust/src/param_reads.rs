//! Callback-local immutable dependency values. Every SDK checkout is checked in
//! before caching a primitive; no host handle, pointer or value survives the call.
use super::*;
use std::cell::RefCell;
const COUNT:usize=Params::EdgeSelector as usize+1;
struct Values<T:Copy>(RefCell<[Option<T>;COUNT]>);
impl<T:Copy> Default for Values<T>{fn default()->Self{Self(RefCell::new([None;COUNT]))}}
impl<T:Copy> Values<T>{
    fn read(&self,id:Params,load:impl FnOnce()->Result<T,ae::Error>)->Result<T,ae::Error>{
        if let Some(value)=self.0.borrow()[id as usize]{return Ok(value);}
        let value=load()?;
        self.0.borrow_mut()[id as usize]=Some(value);
        Ok(value)
    }
}
pub(crate) struct Reads<'a,'p>{
    pub params:&'a ae::Parameters<'p,Params>,pub checkout:bool,
    pub grid:Option<&'a GridArb>,
    floats:Values<f64>,popups:Values<i32>,points:Values<(f64,f64)>,
}
impl<'a,'p> Reads<'a,'p>{
    pub fn new(params:&'a ae::Parameters<'p,Params>,checkout:bool,grid:Option<&'a GridArb>)->Self{
        Self{params,checkout,grid,floats:Values::default(),popups:Values::default(),points:Values::default()}
    }
    pub fn float(&self,id:Params)->Result<f64,ae::Error>{self.floats.read(id,||{
        if self.checkout {checked_float(self.params,id)}else{Ok(self.params.get(id)?.as_float_slider()?.value())}
    })}
    pub fn popup(&self,id:Params)->Result<i32,ae::Error>{self.popups.read(id,||{
        if self.checkout {checked_popup(self.params,id)}else{Ok(self.params.get(id)?.as_popup()?.value())}
    })}
    pub fn point(&self,id:Params)->Result<(f64,f64),ae::Error>{self.points.read(id,||{
        let p=if self.checkout{self.params.checkout(id)?.as_point()?.float_value()?}
            else{self.params.get(id)?.as_point()?.float_value()?};
        Ok((p.x,p.y))
    })}
}
#[cfg(test)] mod tests{
    use super::*;
    #[test]fn repeated_dependencies_load_once_and_new_callbacks_load_again(){
        let calls=std::cell::Cell::new(0);
        let cache=Values::default();
        for _ in 0..100{
            let value=cache.read(Params::TensionRadius,||{calls.set(calls.get()+1);Ok(3.0_f64)}).unwrap();
            assert_eq!(value,3.0);
        }
        assert_eq!(calls.get(),1);
        let next=Values::default();
        assert_eq!(next.read(Params::TensionRadius,||{calls.set(calls.get()+1);Ok(4.0_f64)}),Ok(4.0));
        assert_eq!(calls.get(),2);
    }
    #[test]fn errors_are_not_cached_and_values_retain_exact_bits(){
        let cache=Values::default();
        assert_eq!(cache.read(Params::MinSpacing,||Err::<f64,_>(ae::Error::BadCallbackParameter)),Err(ae::Error::BadCallbackParameter));
        let value=f64::from_bits(0x8000_0000_0000_0000);
        assert_eq!(cache.read(Params::MinSpacing,||Ok(value)).unwrap().to_bits(),value.to_bits());
        assert_eq!(cache.read(Params::MinSpacing,||panic!("already read")).unwrap().to_bits(),value.to_bits());
        assert_eq!(cache.read(Params::WaveAmplitude,||Ok(0.5)),Ok(0.5));
    }
}
