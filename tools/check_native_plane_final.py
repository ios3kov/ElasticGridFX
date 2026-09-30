"""Independent geometry/locality oracle for ae_native_plane_final.jsx captures."""
import json
import math
import sys
from pathlib import Path
from smoke_pixels import read_png

CUSTOM=((30,50),(590,15),(620,425),(65,460))

def project(quad,u,v):
    (x0,y0),(x1,y1),(x2,y2),(x3,y3)=quad
    dx1,dx2,dx3=x1-x2,x3-x2,x0-x1+x2-x3
    dy1,dy2,dy3=y1-y2,y3-y2,y0-y1+y2-y3
    determinant=dx1*dy2-dx2*dy1
    if abs(determinant)<1e-10: raise ValueError('Degenerate basis')
    g=(dx3*dy2-dx2*dy3)/determinant
    h=(dx1*dy3-dx3*dy1)/determinant
    w=g*u+h*v+1
    return (((x1-x0+g*x1)*u+(x3-x0+h*x3)*v+x0)/w,
            ((y1-y0+g*y1)*u+(y3-y0+h*y3)*v+y0)/w)

def compare(folder):
    images={n:read_png(folder/(n+'.png')) for n in
            ('original','neutral','layer-wave','corners-wave','custom-neutral','custom-wave')}
    if any((im.width,im.height)!=(640,480) for im in images.values()):
        raise ValueError('Wrong dimensions')
    a=images['original']
    if max(a.pixels[3::4])-min(a.pixels[3::4])<0.5:
        raise ValueError('Missing nonempty transparent text fixture')
    def error(n,m):return max(abs(x-y) for x,y in zip(images[n].pixels,images[m].pixels))
    identity=max(error('original','neutral'),error('original','custom-neutral'))
    parity=error('layer-wave','corners-wave')
    basis=json.loads((folder/'quad.txt').read_text())
    if len(basis)!=4 or any(len(p)!=2 or not all(math.isfinite(x) for x in p) for p in basis):
        raise ValueError('Invalid quad')
    quad=[project(basis,x/640,y/480) for x,y in CUSTOM]
    exterior=0;interior=0
    b=images['custom-wave']
    for y in range(480):
        for x in range(640):
            distances=[]
            for j in range(4):
                u,v=quad[j],quad[(j+1)%4];dx,dy=v[0]-u[0],v[1]-u[1]
                distances.append((dx*(y-u[1])-dy*(x-u[0]))/math.hypot(dx,dy))
            delta=max(abs(a.pixels[(y*640+x)*4+k]-b.pixels[(y*640+x)*4+k]) for k in range(4))
            if min(distances)<-2:exterior=max(exterior,delta)
            if min(distances)>2 and delta>2/255:interior+=1
    return dict(status='PASS' if identity<=1/255 and parity<=1/255 and exterior<=1/255 and interior>30 else 'FAIL',
                neutral_error=identity,full_domain_parity_error=parity,
                exterior_error=exterior,changed_interior=interior)

if __name__=='__main__':
    root=Path(sys.argv[1]);result={str(d):compare(root/('d'+str(d))) for d in (8,16,32)}
    print(json.dumps(result,indent=2));sys.exit(0 if all(r['status']=='PASS' for r in result.values()) else 1)
