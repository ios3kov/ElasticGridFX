// FSTR native plane v3
var r=thisLayer.sourceRectAtTime(time,false);
var q=[r.left@RIGHT@,r.top@BOTTOM@,0];
var local=thisProperty.propertyGroup(1)(1).value===3 && thisLayer.transform.position.value.length!==3;
var p=local?q:thisLayer.toComp(q);
[p[0],p[1]];
