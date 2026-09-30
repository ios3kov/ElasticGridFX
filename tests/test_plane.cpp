#include "core/PlaneTransform.h"
#include <cassert>
#include <cmath>
#include <limits>
using namespace elasticgrid;
void near(PlanePoint a,PlanePoint b,double eps=1e-8) {
    assert(std::abs(a.x-b.x)<eps && std::abs(a.y-b.y)<eps);
}
int main() {
    using Quad=std::array<PlanePoint,4>;
    for(auto corners : {Quad{{{0,0},{1,0},{1,1},{0,1}}},
                         Quad{{{0,0},{2,0},{1.5,1},{.5,1}}},
                         Quad{{{2,0},{0,0},{.5,1},{1.5,1}}},
                         Quad{{{1e9,1e9},{1e9+200,1e9+20},{1e9+150,1e9+100},{1e9+30,1e9+90}}}}) {
        auto h=PlaneTransform::fromCorners(corners); assert(h);
        Quad local{{{0,0},{1,0},{1,1},{0,1}}};
        for(int i=0;i<4;++i) {auto p=h->toSurface(local[i]); assert(p); near(*p,corners[i],1e-6);}
        for(int x=0;x<=20;++x) for(int y=0;y<=20;++y) {
            PlanePoint q{x/20.0,y/20.0}; auto p=h->toSurface(q);assert(p);
            auto back=h->toLocal(*p);assert(back);near(*back,q,1e-6);
        }
    }
    auto h=PlaneTransform::fromCorners(Quad{{{0,0},{2,0},{1.5,1},{.5,1}}}); assert(h);
    near(*h->toSurface({.5,.5}),{1,2.0/3});
    assert(!h->toSurface({0,-1})); // projective horizon
    assert(!h->toLocal({std::numeric_limits<double>::infinity(),0}));
    for(auto bad : {Quad{{{0,0},{1,1},{1,0},{0,1}}},Quad{{{0,0},{1,0},{.2,.2},{0,1}}},
                    Quad{{{0,0},{1,0},{1,0},{0,1}}},Quad{{{0,0},{1,0},{1,1e-12},{0,1e-12}}}})
        assert(!PlaneTransform::fromCorners(bad));
    Quad invalid{{{0,0},{1,0},{1,1},{0,1}}}; invalid[0].x=std::numeric_limits<double>::quiet_NaN();
    assert(!PlaneTransform::fromCorners(invalid));
}
