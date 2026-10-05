// FSTR native plane v4
var k=1;var three=thisLayer.transform.position.value.length===3;
var text=false;if(three){try{var t=thisLayer.text.sourceText.value;text=true;}catch(e){}}
var mode=thisProperty.propertyGroup(1)(1).value;
if(mode===1){k=three?(text?7:6):5;}
else if(three){k=text?2:3;}
else if(mode===3){k=4;}
k;
