#include "PlaneTransform.h"
#include <algorithm>
#include <cmath>

namespace elasticgrid {
namespace {
constexpr double tolerance = 1e-10; // normalized geometry, not screen pixels
bool finite(PlanePoint p) { return std::isfinite(p.x) && std::isfinite(p.y); }
std::optional<PlanePoint> project(const std::array<double,9>& m, PlanePoint p) {
    if (!finite(p)) return std::nullopt;
    const double w = m[6]*p.x + m[7]*p.y + m[8];
    const double bound = std::abs(m[6]*p.x)+std::abs(m[7]*p.y)+std::abs(m[8]);
    if (!std::isfinite(w) || std::abs(w) <= tolerance*bound) return std::nullopt;
    PlanePoint q{(m[0]*p.x+m[1]*p.y+m[2])/w, (m[3]*p.x+m[4]*p.y+m[5])/w};
    return finite(q) ? std::optional<PlanePoint>(q) : std::nullopt;
}
}

std::optional<PlaneTransform> PlaneTransform::fromCorners(const std::array<PlanePoint,4>& corners) {
    PlaneTransform result;
    for (auto p : corners) if (!finite(p)) return std::nullopt;
    result.origin_ = corners[0];
    result.scale_ = 0;
    for (auto p : corners) result.scale_ = std::max({result.scale_,
        std::abs(p.x-corners[0].x), std::abs(p.y-corners[0].y)});
    if (!std::isfinite(result.scale_) || result.scale_ == 0) return std::nullopt;
    std::array<PlanePoint,4> p;
    for (int i=0;i<4;++i) p[i]={(corners[i].x-result.origin_.x)/result.scale_,
                               (corners[i].y-result.origin_.y)/result.scale_};
    // Require a strict convex quad. Mirrored winding is valid; folds are not.
    double winding=0;
    for (int i=0;i<4;++i) {
        auto a=p[i], b=p[(i+1)%4], c=p[(i+2)%4];
        double cross=(b.x-a.x)*(c.y-b.y)-(b.y-a.y)*(c.x-b.x);
        if (std::abs(cross)<=tolerance || (i && cross*winding<=0)) return std::nullopt;
        winding=cross;
    }
    // Solve the normalized eight homography equations with pivoting.
    double a[8][9]{};
    constexpr double u[4]={0,1,1,0}, v[4]={0,0,1,1};
    for (int i=0;i<4;++i) {
        a[2*i][0]=u[i]; a[2*i][1]=v[i]; a[2*i][2]=1;
        a[2*i][6]=-p[i].x*u[i]; a[2*i][7]=-p[i].x*v[i]; a[2*i][8]=p[i].x;
        a[2*i+1][3]=u[i]; a[2*i+1][4]=v[i]; a[2*i+1][5]=1;
        a[2*i+1][6]=-p[i].y*u[i]; a[2*i+1][7]=-p[i].y*v[i]; a[2*i+1][8]=p[i].y;
    }
    for (int col=0;col<8;++col) {
        int pivot=col;
        for (int row=col+1;row<8;++row) if(std::abs(a[row][col])>std::abs(a[pivot][col])) pivot=row;
        if(std::abs(a[pivot][col])<=tolerance) return std::nullopt;
        for(int j=0;j<9;++j) std::swap(a[col][j],a[pivot][j]);
        double divisor=a[col][col];
        for(int j=col;j<9;++j) a[col][j]/=divisor;
        for(int row=0;row<8;++row) if(row!=col) {
            double factor=a[row][col];
            for(int j=col;j<9;++j) a[row][j]-=factor*a[col][j];
        }
    }
    auto& m=result.forward_;
    for(int i=0;i<8;++i) m[i]=a[i][8];
    m[8]=1;
    for(int i=0;i<4;++i) if(m[6]*u[i]+m[7]*v[i]+1<=tolerance) return std::nullopt;
    // Adjugate: homogeneous scale cancels during projection.
    result.inverse_={m[4]*m[8]-m[5]*m[7],m[2]*m[7]-m[1]*m[8],m[1]*m[5]-m[2]*m[4],
        m[5]*m[6]-m[3]*m[8],m[0]*m[8]-m[2]*m[6],m[2]*m[3]-m[0]*m[5],
        m[3]*m[7]-m[4]*m[6],m[1]*m[6]-m[0]*m[7],m[0]*m[4]-m[1]*m[3]};
    for(double x:m) if(!std::isfinite(x)) return std::nullopt;
    for(double x:result.inverse_) if(!std::isfinite(x)) return std::nullopt;
    return result;
}
std::optional<PlanePoint> PlaneTransform::toSurface(PlanePoint local) const {
    auto p=project(forward_,local);
    if(!p) return std::nullopt;
    PlanePoint q{p->x*scale_+origin_.x,p->y*scale_+origin_.y};
    return finite(q) ? std::optional<PlanePoint>(q) : std::nullopt;
}
std::optional<PlanePoint> PlaneTransform::toLocal(PlanePoint surface) const {
    return project(inverse_,{(surface.x-origin_.x)/scale_,(surface.y-origin_.y)/scale_});
}
} // namespace elasticgrid
