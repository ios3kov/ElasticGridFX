#include "bridge/plane_ffi.h"
#include <algorithm>
#include <chrono>
#include <bit>
#include <limits>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <string>
#include <type_traits>
#include <vector>

namespace {
std::int32_t neverAbort(void*) noexcept { return 0; }

template<typename T>
int benchmark(int width, int height, int frames, int depth, const std::string& mode,
              bool sparse, const std::string& dump, bool nonfinite) {
    const int sw=sparse?std::max(1,width/3):width, sh=sparse?std::max(1,height/3):height;
    const int source_stride=sw*4+8, output_stride=width*4+8;
    std::vector<T> input(static_cast<std::size_t>(source_stride)*sh);
    std::vector<T> output(static_cast<std::size_t>(output_stride)*height,T(17));
    for(int y=0;y<sh;++y) for(int x=0;x<sw;++x) for(int c=0;c<4;++c) {
        const unsigned v=(static_cast<unsigned>(x)*17+static_cast<unsigned>(y)*31+
                          static_cast<unsigned>(c)*53)%251;
        if constexpr(std::is_same_v<T,float>)
            input[static_cast<std::size_t>(y)*source_stride+x*4+c]=c==3?float(v)/250:float(v)/50-2;
        else input[static_cast<std::size_t>(y)*source_stride+x*4+c]=
            static_cast<T>(v*(depth==16?128:1));
    }
    if constexpr(std::is_same_v<T,float>) {
        if(nonfinite) {
            input[0]=std::bit_cast<float>(0x80000000u);
            if(input.size()>17) {
                input[9]=std::numeric_limits<float>::infinity();
                input[17]=std::bit_cast<float>(0x7fc00123u);
            }
        }
    }
    std::vector<float> columns{0,.12f,.30f,.51f,.70f,.85f,1}, rows{0,.18f,.40f,.61f,.79f,1};
    if(mode=="identity") {
        for(std::size_t i=0;i<columns.size();++i) columns[i]=float(i)/float(columns.size()-1);
        for(std::size_t i=0;i<rows.size();++i) rows[i]=float(i)/float(rows.size()-1);
    }
    EgPlaneFrame frame{};
    const double w=width-1,h=height-1;
    const double flat[8]={0,0,w,0,w,h,0,h};
    std::copy(flat,flat+8,frame.corners);
    if(mode=="perspective") {
        const double quad[8]={.05*w,.08*h,.9*w,.03*h,.95*w,.92*h,.13*w,.88*h};
        std::copy(quad,quad+8,frame.corners);
    }
    frame.columns=columns.data(); frame.rows=rows.data();
    frame.column_count=static_cast<int>(columns.size()); frame.row_count=static_cast<int>(rows.size());
    frame.surface_units_x=frame.surface_units_y=1;
    frame.canvas_width=width; frame.canvas_height=height;
    frame.source_x=sparse?width/5:0; frame.source_y=sparse?height/5:0;
    frame.easing=.4f; frame.easing_distance=.25f; frame.abort_fn=&neverAbort;
    EgPlaneImage src{input.data(),source_stride*static_cast<std::ptrdiff_t>(sizeof(T)),sw,sh};
    EgPlaneImage dst{output.data(),output_stride*static_cast<std::ptrdiff_t>(sizeof(T)),width,height};
    EgPlaneReport report{};
    auto render=[&] {
        const int result=mode=="layer"?eg_render_plane_layer(&src,&dst,depth,&frame,&report,1,0):
            eg_render_plane_region(&src,&dst,depth,&frame,&report,1,0);
        if(result) throw std::runtime_error("plane bridge render failed: "+std::to_string(result));
    };
    std::vector<double> times;
    const auto cold_start=std::chrono::steady_clock::now(); render();
    const double cold=std::chrono::duration<double,std::milli>(std::chrono::steady_clock::now()-cold_start).count();
    render(); render();
    for(int i=0;i<frames;++i) {
        const auto start=std::chrono::steady_clock::now(); render();
        times.push_back(std::chrono::duration<double,std::milli>(std::chrono::steady_clock::now()-start).count());
    }
    if(!dump.empty()) {
        std::ofstream file(dump,std::ios::binary|std::ios::trunc);
        file.write(reinterpret_cast<const char*>(output.data()),
                   static_cast<std::streamsize>(output.size()*sizeof(T)));
        if(!file) throw std::runtime_error("cannot write pixel evidence");
    }
    std::cout<<"{\"scope\":\"native plane bridge, NOT After Effects or RAM Preview\","
             <<"\"width\":"<<width<<",\"height\":"<<height<<",\"depth\":"<<depth
             <<",\"quality\":\"Final Bicubic\",\"mode\":\""<<mode<<"\",\"sparse\":"
             <<(sparse?"true":"false")<<",\"nonfinite_input\":"<<(nonfinite?"true":"false")
             <<",\"abort_polling\":true,\"cold_ms\":"<<cold<<",\"frame_ms\":[";
    for(std::size_t i=0;i<times.size();++i) std::cout<<(i?",":"")<<times[i];
    std::cout<<"],\"outside_pixels\":"<<report.outside_pixels
             <<",\"invalid_projection_pixels\":"<<report.invalid_projection_pixels<<"}\n";
    return 0;
}
}
int main(int argc,char** argv) {
    try {
        const int w=argc>1?std::stoi(argv[1]):1920,h=argc>2?std::stoi(argv[2]):1080;
        const int frames=argc>3?std::stoi(argv[3]):5,depth=argc>4?std::stoi(argv[4]):32;
        const std::string mode=argc>5?argv[5]:"region";
        const bool sparse=argc>6 && std::string(argv[6])=="sparse";
        const bool nonfinite=argc>8 && std::string(argv[8])=="nonfinite";
        if(w<2 || h<2 || w>8192 || h>8192 || frames<1 || frames>100 ||
           (depth!=8 && depth!=16 && depth!=32) || (nonfinite && depth!=32) ||
           (mode!="region" && mode!="layer" && mode!="perspective" && mode!="identity"))
            throw std::invalid_argument("usage: bench_plane width height frames depth region|layer|perspective|identity dense|sparse [pixel_dump] [finite|nonfinite (32 bpc)]");
        const std::string dump=argc>7?argv[7]:"";
        if(depth==8) return benchmark<std::uint8_t>(w,h,frames,depth,mode,sparse,dump,nonfinite);
        if(depth==16) return benchmark<std::uint16_t>(w,h,frames,depth,mode,sparse,dump,nonfinite);
        return benchmark<float>(w,h,frames,depth,mode,sparse,dump,nonfinite);
    } catch(const std::exception& error) {std::cerr<<error.what()<<'\n';return 2;}
}
