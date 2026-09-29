#include "core/PlaneWarp.h"
#include "core/WarpMath.h"
#include <cassert>
#include <cmath>
#include <limits>
using namespace elasticgrid;
void near(PlanePoint a,PlanePoint b) {
    assert(std::abs(a.x-b.x)<1e-6 && std::abs(a.y-b.y)<1e-6);
}
int main() {
    // H(u,v)=(u/(1+v), (u+2v)/(1+v)): columns run diagonally on screen.
    auto plane=PlaneTransform::fromCorners({{{0,0},{1,1},{.5,1.5},{0,1}}});assert(plane);
    auto warp=PlaneWarp::prepare(*plane,{0,.75f,1},{0,.5f,1});assert(warp);
    auto destination=plane->toSurface({.75,.5});assert(destination);
    auto result=warp->sourceFor(*destination);
    assert(result.status==PlaneMapStatus::Mapped && result.source);
    near(*result.source,{1.0/3,1}); // old local guide .5 now lies at local .75
    // A screen-horizontal-only warp would incorrectly leave y unchanged.
    assert(std::abs(result.source->y-destination->y)>.1);
    auto outside=warp->sourceFor(*plane->toSurface({1.1,.5}));
    assert(outside.status==PlaneMapStatus::OutsidePlane && !outside.source);
    assert(warp->sourceFor({0,2}).status==PlaneMapStatus::InvalidProjection); // inverse horizon
    assert(warp->sourceFor({NAN,0}).status==PlaneMapStatus::InvalidProjection);

    auto identity=PlaneWarp::prepare(*plane,{0,.5f,1},{0,.5f,1},1,1);assert(identity);
    for(int i=0;i<=20;++i) for(int j=0;j<=20;++j) {
        auto q=plane->toSurface({i/20.0,j/20.0});assert(q);
        auto same=identity->sourceFor(*q);assert(same.source);
        assert(same.source->x==q->x && same.source->y==q->y);
    }
    auto unit=PlaneTransform::fromCorners({{{0,0},{1,0},{1,1},{0,1}}});assert(unit);
    for(float easing:{0.f,.5f,1.f}) {
        auto flat=PlaneWarp::prepare(*unit,{0,.75f,1},{0,.25f,1},easing);assert(flat);
        for(int i=0;i<=20;++i) for(int j=0;j<=20;++j) {
            float x=i/20.f,y=j/20.f;
            auto p=flat->sourceFor({x,y});assert(p.source);
            near(*p.source,{inverseMapNormalized(x,{0,.75f,1},easing,.25f),
                            inverseMapNormalized(y,{0,.25f,1},easing,.25f)});
        }
    }
    for(auto axis:{std::vector<float>{}, {0,0,1}, {0,1,.5f,1}, {.1f,1}, {0,.9f}, {0,NAN,1}})
        assert(!PlaneWarp::prepare(*plane,axis,{0,1}));
    assert(!PlaneWarp::prepare(*plane,{0,1},{0,1},-1));
    assert(!PlaneWarp::prepare(*plane,{0,1},{0,1},0,INFINITY));
    std::vector<float> owned{0,.75f,1};
    auto snapshot=PlaneWarp::prepare(*plane,owned,{0,.5f,1});assert(snapshot);
    owned[1]=.1f;near(*snapshot->sourceFor(*destination).source,*result.source);
}
