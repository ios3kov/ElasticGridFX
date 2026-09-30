"""Independent native frame identity/locality check, not direction/picking acceptance."""
import json
import sys
from pathlib import Path
from smoke_pixels import read_png

def check(folder):
    report=(folder/'render-v3.txt').read_text()
    if not report.startswith('CAPTURED kind=2'):
        raise ValueError('Native comp-space binding not established')
    quad=json.loads(report.splitlines()[1])
    images=[read_png(folder/(n+'-v3.png')) for n in ('original','neutral','deformed')]
    a,b,c=images
    if any((i.width,i.height)!=(640,480) for i in images):
        raise ValueError('Wrong native fixture dimensions')
    identity=max(abs(x-y) for x,y in zip(a.pixels,b.pixels))
    inside_changes=0
    exterior_error=0.0
    for y in range(a.height):
        for x in range(a.width):
            distances=[]
            for j in range(4):
                u,v=quad[j],quad[(j+1)%4]
                dx,dy=v[0]-u[0],v[1]-u[1]
                distances.append((dx*(y-u[1])-dy*(x-u[0]))/(dx*dx+dy*dy)**0.5)
            delta=max(abs(a.pixels[(y*a.width+x)*4+k]-c.pixels[(y*a.width+x)*4+k]) for k in range(4))
            if min(distances)>2 and delta>2/255:
                inside_changes+=1
            if min(distances)<-2:
                exterior_error=max(exterior_error,delta)
    return dict(status='PASS' if identity<=1/255 and exterior_error<=1/255 and inside_changes>30 else 'FAIL',
                neutral_max_error=identity,exterior_max_error=exterior_error,
                changed_interior_pixels=inside_changes,
                scope='native text identity and region locality only; project depth recorded by producer')

if __name__=='__main__':
    result=check(Path(sys.argv[1]));print(json.dumps(result,indent=2))
    sys.exit(0 if result['status']=='PASS' else 1)
