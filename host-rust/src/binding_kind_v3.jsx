// FSTR native plane v3
var k=1;
if(thisLayer.transform.position.value.length===3){
    k=3;
    try{var t=thisLayer.text.sourceText.value;k=2;}catch(e){}
}else if(thisProperty.propertyGroup(1)(1).value===3){k=4;}
k;
