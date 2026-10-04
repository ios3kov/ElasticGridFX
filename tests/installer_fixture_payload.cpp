#include "Payload.h"
namespace fstr::payload {
static const unsigned char data[]="MZ com.elasticgrid.fx.warp ElasticGridBuildID=EGFX-111111111111111111111111";
const File files[]={{"FSTR Stretch.aex",data,sizeof(data)-1,"b62b97a5fea186db5470d9e5069cf38879cef692c3780a5472aa7f9c0cb4e8dd",false}};
const std::size_t count=1;
const char version[]="0.9.4",buildId[]="EGFX-111111111111111111111111",commit[]="fixture",platform[]="windows-x64";
}
