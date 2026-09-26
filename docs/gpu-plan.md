# Metal implementation — v0.6

## Host integration

The Mac path uses After Effects GPU Smart Render rather than creating a separate rendering pipeline around CPU images.
During GPU setup the plugin receives AE's Metal device/command queue. During GPU Smart Render it obtains the input and
output GPU-world buffers directly from AE.

## Geometry/sampling parity

The GPU does **not** implement a second inverse-warp algorithm.

Per frame on CPU:
1. evaluate the current keyframed guide state and wave;
2. build inverse X/Y mapping;
3. produce the exact sampling plan:
   - Bilinear: two indices + fraction per axis sample;
   - Bicubic: four indices + four Catmull-Rom weights per axis sample.

On Metal:
1. read one X-plan entry and one Y-plan entry;
2. sample the AE float4 buffer using those exact indices/weights;
3. write one float4 output pixel.

This keeps CPU/Metal geometry and Final-quality filtering aligned.

## Allocation policy

- Rust sampling-plan vectors are thread-local and reused after warmup.
- Metal shared plan buffers are pooled and safe for concurrent MFR calls.
- full image input/output buffers are AE-owned; no CPU image upload/download is introduced.

## Acceptance gate

Metal remains blocked from packaging if any of these fail:

- real CPU/Metal image parity for Bilinear and Bicubic;
- Clamp, Wrap and Mirror parity;
- shader/pipeline creation;
- 4K Metal benchmark execution;
- macOS AE host compile;
- bundle signing/verification.

The real AE runtime test follows packaging and is not claimed from the Linux development environment.
