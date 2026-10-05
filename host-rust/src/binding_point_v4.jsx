// FSTR native plane v4
var r=thisLayer.sourceRectAtTime(time,false);
var q=[r.left@RIGHT@,r.top@BOTTOM@,0];
var mode=thisProperty.propertyGroup(1)(1).value;
var three=thisLayer.transform.position.value.length===3;
var text=false;if(three){try{var t=thisLayer.text.sourceText.value;text=true;}catch(e){}}
var p;
if(mode===1){
    var c=[@COMPX@,@COMPY@];
    p=three?(text?c:thisLayer.fromCompToSurface(c)):thisLayer.fromComp(c);
}else{
    var local=mode===3&&!three;
    p=local?q:thisLayer.toComp(q);
}
[p[0],p[1]];
