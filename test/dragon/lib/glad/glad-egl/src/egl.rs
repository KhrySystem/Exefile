pub use self::types::*;
pub use self::enumerations::*;
pub use self::functions::*;

use std::os::raw::c_void;


#[derive(Copy, Clone)]
struct FnPtr {
    ptr: *const c_void,
    is_loaded: bool
}

#[allow(dead_code)]
impl FnPtr {
    fn new(ptr: *const c_void) -> FnPtr {
        if !ptr.is_null() {
            FnPtr { ptr, is_loaded: true }
        } else {
            FnPtr { ptr: FnPtr::not_initialized as *const c_void, is_loaded: false }
        }
    }

    fn set_ptr(&mut self, ptr: *const c_void) {
        *self = Self::new(ptr);
    }
    
    fn aliased(&mut self, other: &FnPtr) {
        if !self.is_loaded && other.is_loaded {
            *self = *other;
        }
    }

    #[inline(never)]
    fn not_initialized() -> ! { panic!("egl: function not initialized") }
}

unsafe impl Sync for FnPtr {}
unsafe impl Send for FnPtr {}

pub mod types {
#![allow(dead_code, non_camel_case_types, non_snake_case)]
#![allow(non_camel_case_types)]

use std;

// see khrplatform.h for these types
pub type khronos_int8_t  = i8;
pub type khronos_uint8_t = u8;
pub type khronos_int16_t = i16;
pub type khronos_uint16_t = u16;
pub type khronos_int32_t = i32;
pub type khronos_uint32_t = u32;
pub type khronos_int64_t = i64;
pub type khronos_uint64_t = u64;
pub type khronos_intptr_t = isize;
pub type khronos_uintptr_t = usize;
pub type khronos_ssize_t  = isize;
pub type khronos_usize_t  = usize;
pub type khronos_float_t = std::os::raw::c_float;
pub type khronos_time_ns_t = u64;
pub type khronos_stime_nanoseconds_t = i64;
pub type khronos_utime_nanoseconds_t = u64;

pub type EGLint = khronos_int32_t;

// TODO replace based on platform, see eglplatform.h
#[cfg(target_os = "macos")]      pub type EGLNativeDisplayType = i32;
#[cfg(not(target_os = "macos"))] pub type EGLNativeDisplayType = *mut std::os::raw::c_void;
pub type EGLNativeWindowType = *mut std::os::raw::c_void;
pub type EGLNativePixmapType = *mut std::os::raw::c_void;

// EGL types
pub type EGLBoolean = std::os::raw::c_uint;
pub type EGLenum = std::os::raw::c_uint;

pub type EGLClientBuffer = *mut std::os::raw::c_void;
pub type EGLConfig = *mut std::os::raw::c_void;
pub type EGLContext = *mut std::os::raw::c_void;
pub type EGLDeviceEXT = *mut std::os::raw::c_void;
pub type EGLDisplay = *mut std::os::raw::c_void;
pub type EGLImage = *mut std::os::raw::c_void;
pub type EGLImageKHR = *mut std::os::raw::c_void;
pub type EGLLabelKHR = *mut std::os::raw::c_void;
pub type EGLObjectKHR = *mut std::os::raw::c_void;
pub type EGLOutputLayerEXT = *mut std::os::raw::c_void;
pub type EGLOutputPortEXT = *mut std::os::raw::c_void;
pub type EGLStreamKHR = *mut std::os::raw::c_void;
pub type EGLSurface = *mut std::os::raw::c_void;
pub type EGLSync = *mut std::os::raw::c_void;
pub type EGLSyncKHR = *mut std::os::raw::c_void;
pub type EGLSyncNV = *mut std::os::raw::c_void;

pub type EGLAttrib = isize;
pub type EGLAttribKHR = isize;
pub enum __eglMustCastToProperFunctionPointerType_fn {}
pub type __eglMustCastToProperFunctionPointerType = *mut __eglMustCastToProperFunctionPointerType_fn;
pub type EGLNativeFileDescriptorKHR = std::os::raw::c_int;
pub type EGLnsecsANDROID = khronos_stime_nanoseconds_t;
pub type EGLsizeiANDROID = khronos_ssize_t;
pub type EGLTimeKHR = khronos_utime_nanoseconds_t;
pub type EGLTime = khronos_utime_nanoseconds_t;
pub type EGLTimeNV = khronos_utime_nanoseconds_t;
pub type EGLuint64KHR = khronos_uint64_t;
pub type EGLuint64NV = khronos_utime_nanoseconds_t;
pub struct AHardwareBuffer;

pub type EGLSetBlobFuncANDROID = extern "system" fn (
    *const std::os::raw::c_void,
    EGLsizeiANDROID,
    *const std::os::raw::c_void,
    EGLsizeiANDROID
) -> ();
pub type EGLGetBlobFuncANDROID = extern "system" fn (
    *const std::os::raw::c_void,
    EGLsizeiANDROID,
    *mut std::os::raw::c_void,
    EGLsizeiANDROID
) -> EGLsizeiANDROID;
pub type EGLDEBUGPROCKHR = extern "system" fn (
    error: EGLenum,
    command: *mut std::os::raw::c_char,
    messageType: EGLint,
    threadLabel: EGLLabelKHR,
    objectLabel: EGLLabelKHR,
    message: *mut std::os::raw::c_char
) -> ();


#[repr(C)]
#[derive(Copy, Clone)]
pub struct EGLClientPixmapHI {
    pData: *const std::os::raw::c_void,
    iWidth: EGLint,
    iHeight: EGLint,
    iStride: EGLint,
}

pub type wl_display = std::os::raw::c_void;
pub type wl_surface = std::os::raw::c_void;
pub type wl_buffer = std::os::raw::c_void;
pub type wl_resource = std::os::raw::c_void;

}

pub mod enumerations {
    #![allow(dead_code, non_upper_case_globals, unused_imports)]

    use std::os::raw::*;
    use super::types::*;

    pub const ALPHA_FORMAT: c_uint = 0x3088;
    pub const ALPHA_FORMAT_NONPRE: c_uint = 0x308B;
    pub const ALPHA_FORMAT_PRE: c_uint = 0x308C;
    pub const ALPHA_MASK_SIZE: c_uint = 0x303E;
    pub const ALPHA_SIZE: c_uint = 0x3021;
    pub const ALREADY_SIGNALED_NV: c_uint = 0x30EA;
    pub const AUTO_STEREO_NV: c_uint = 0x3136;
    pub const BACK_BUFFER: c_uint = 0x3084;
    pub const BAD_ACCESS: c_uint = 0x3002;
    pub const BAD_ALLOC: c_uint = 0x3003;
    pub const BAD_ATTRIBUTE: c_uint = 0x3004;
    pub const BAD_CONFIG: c_uint = 0x3005;
    pub const BAD_CONTEXT: c_uint = 0x3006;
    pub const BAD_CURRENT_SURFACE: c_uint = 0x3007;
    pub const BAD_DEVICE_EXT: c_uint = 0x322B;
    pub const BAD_DISPLAY: c_uint = 0x3008;
    pub const BAD_MATCH: c_uint = 0x3009;
    pub const BAD_NATIVE_PIXMAP: c_uint = 0x300A;
    pub const BAD_NATIVE_WINDOW: c_uint = 0x300B;
    pub const BAD_OUTPUT_LAYER_EXT: c_uint = 0x322D;
    pub const BAD_OUTPUT_PORT_EXT: c_uint = 0x322E;
    pub const BAD_PARAMETER: c_uint = 0x300C;
    pub const BAD_STATE_KHR: c_uint = 0x321C;
    pub const BAD_STREAM_KHR: c_uint = 0x321B;
    pub const BAD_SURFACE: c_uint = 0x300D;
    pub const BIND_TO_TEXTURE_RGB: c_uint = 0x3039;
    pub const BIND_TO_TEXTURE_RGBA: c_uint = 0x303A;
    pub const BITMAP_ORIGIN_KHR: c_uint = 0x30C8;
    pub const BITMAP_PITCH_KHR: c_uint = 0x30C7;
    pub const BITMAP_PIXEL_ALPHA_OFFSET_KHR: c_uint = 0x30CC;
    pub const BITMAP_PIXEL_BLUE_OFFSET_KHR: c_uint = 0x30CB;
    pub const BITMAP_PIXEL_GREEN_OFFSET_KHR: c_uint = 0x30CA;
    pub const BITMAP_PIXEL_LUMINANCE_OFFSET_KHR: c_uint = 0x30CD;
    pub const BITMAP_PIXEL_RED_OFFSET_KHR: c_uint = 0x30C9;
    pub const BITMAP_PIXEL_SIZE_KHR: c_uint = 0x3110;
    pub const BITMAP_POINTER_KHR: c_uint = 0x30C6;
    pub const BLUE_SIZE: c_uint = 0x3022;
    pub const BOTTOM_NV: c_uint = 0x336E;
    pub const BUFFER_AGE_EXT: c_uint = 0x313D;
    pub const BUFFER_AGE_KHR: c_uint = 0x313D;
    pub const BUFFER_DESTROYED: c_uint = 0x3095;
    pub const BUFFER_PRESERVED: c_uint = 0x3094;
    pub const BUFFER_SIZE: c_uint = 0x3020;
    pub const CLIENT_APIS: c_uint = 0x308D;
    pub const CLIENT_PIXMAP_POINTER_HI: c_uint = 0x8F74;
    pub const CL_EVENT_HANDLE: c_uint = 0x309C;
    pub const CL_EVENT_HANDLE_KHR: c_uint = 0x309C;
    pub const COLORSPACE: c_uint = 0x3087;
    pub const COLORSPACE_LINEAR: c_uint = 0x308A;
    pub const COLORSPACE_sRGB: c_uint = 0x3089;
    pub const COLOR_ARGB_HI: c_uint = 0x8F73;
    pub const COLOR_BUFFER_TYPE: c_uint = 0x303F;
    pub const COLOR_COMPONENT_TYPE_EXT: c_uint = 0x3339;
    pub const COLOR_COMPONENT_TYPE_FIXED_EXT: c_uint = 0x333A;
    pub const COLOR_COMPONENT_TYPE_FLOAT_EXT: c_uint = 0x333B;
    pub const COLOR_COMPONENT_TYPE_INTEGER_ARM: c_uint = 0x3288;
    pub const COLOR_COMPONENT_TYPE_UNSIGNED_INTEGER_ARM: c_uint = 0x3287;
    pub const COLOR_FORMAT_HI: c_uint = 0x8F70;
    pub const COLOR_RGBA_HI: c_uint = 0x8F72;
    pub const COLOR_RGB_HI: c_uint = 0x8F71;
    pub const COMPOSITE_DEADLINE_ANDROID: c_uint = 0x3431;
    pub const COMPOSITE_INTERVAL_ANDROID: c_uint = 0x3432;
    pub const COMPOSITE_TO_PRESENT_LATENCY_ANDROID: c_uint = 0x3433;
    pub const COMPOSITION_LATCH_TIME_ANDROID: c_uint = 0x3436;
    pub const COMPOSITOR_DROP_NEWEST_FRAME_EXT: c_uint = 0x3462;
    pub const COMPOSITOR_KEEP_NEWEST_FRAME_EXT: c_uint = 0x3463;
    pub const CONDITION_SATISFIED: c_uint = 0x30F6;
    pub const CONDITION_SATISFIED_KHR: c_uint = 0x30F6;
    pub const CONDITION_SATISFIED_NV: c_uint = 0x30EC;
    pub const CONFIG_CAVEAT: c_uint = 0x3027;
    pub const CONFIG_ID: c_uint = 0x3028;
    pub const CONFIG_SELECT_GROUP_EXT: c_uint = 0x34C0;
    pub const CONFORMANT: c_uint = 0x3042;
    pub const CONFORMANT_KHR: c_uint = 0x3042;
    pub const CONSUMER_ACQUIRE_TIMEOUT_USEC_KHR: c_uint = 0x321E;
    pub const CONSUMER_AUTO_ORIENTATION_NV: c_uint = 0x3369;
    pub const CONSUMER_FRAME_KHR: c_uint = 0x3213;
    pub const CONSUMER_LATENCY_USEC_KHR: c_uint = 0x3210;
    pub const CONSUMER_MAX_FRAME_HINT_NV: c_uint = 0x3338;
    pub const CONSUMER_METADATA_NV: c_uint = 0x3254;
    pub const CONTEXT_CLIENT_TYPE: c_uint = 0x3097;
    pub const CONTEXT_CLIENT_VERSION: c_uint = 0x3098;
    pub const CONTEXT_FLAGS_KHR: c_uint = 0x30FC;
    pub const CONTEXT_LOST: c_uint = 0x300E;
    pub const CONTEXT_MAJOR_VERSION: c_uint = 0x3098;
    pub const CONTEXT_MAJOR_VERSION_KHR: c_uint = 0x3098;
    pub const CONTEXT_MINOR_VERSION: c_uint = 0x30FB;
    pub const CONTEXT_MINOR_VERSION_KHR: c_uint = 0x30FB;
    pub const CONTEXT_OPENGL_COMPATIBILITY_PROFILE_BIT: c_uint = 0x00000002;
    pub const CONTEXT_OPENGL_COMPATIBILITY_PROFILE_BIT_KHR: c_uint = 0x00000002;
    pub const CONTEXT_OPENGL_CORE_PROFILE_BIT: c_uint = 0x00000001;
    pub const CONTEXT_OPENGL_CORE_PROFILE_BIT_KHR: c_uint = 0x00000001;
    pub const CONTEXT_OPENGL_DEBUG: c_uint = 0x31B0;
    pub const CONTEXT_OPENGL_DEBUG_BIT_KHR: c_uint = 0x00000001;
    pub const CONTEXT_OPENGL_FORWARD_COMPATIBLE: c_uint = 0x31B1;
    pub const CONTEXT_OPENGL_FORWARD_COMPATIBLE_BIT_KHR: c_uint = 0x00000002;
    pub const CONTEXT_OPENGL_NO_ERROR_KHR: c_uint = 0x31B3;
    pub const CONTEXT_OPENGL_PROFILE_MASK: c_uint = 0x30FD;
    pub const CONTEXT_OPENGL_PROFILE_MASK_KHR: c_uint = 0x30FD;
    pub const CONTEXT_OPENGL_RESET_NOTIFICATION_STRATEGY: c_uint = 0x31BD;
    pub const CONTEXT_OPENGL_RESET_NOTIFICATION_STRATEGY_EXT: c_uint = 0x3138;
    pub const CONTEXT_OPENGL_RESET_NOTIFICATION_STRATEGY_KHR: c_uint = 0x31BD;
    pub const CONTEXT_OPENGL_ROBUST_ACCESS: c_uint = 0x31B2;
    pub const CONTEXT_OPENGL_ROBUST_ACCESS_BIT_KHR: c_uint = 0x00000004;
    pub const CONTEXT_OPENGL_ROBUST_ACCESS_EXT: c_uint = 0x30BF;
    pub const CONTEXT_PRIORITY_HIGH_IMG: c_uint = 0x3101;
    pub const CONTEXT_PRIORITY_LEVEL_IMG: c_uint = 0x3100;
    pub const CONTEXT_PRIORITY_LOW_IMG: c_uint = 0x3103;
    pub const CONTEXT_PRIORITY_MEDIUM_IMG: c_uint = 0x3102;
    pub const CONTEXT_PRIORITY_REALTIME_NV: c_uint = 0x3357;
    pub const CONTEXT_RELEASE_BEHAVIOR_FLUSH_KHR: c_uint = 0x2098;
    pub const CONTEXT_RELEASE_BEHAVIOR_KHR: c_uint = 0x2097;
    pub const CONTEXT_RELEASE_BEHAVIOR_NONE_KHR: c_uint = 0;
    pub const CORE_NATIVE_ENGINE: c_uint = 0x305B;
    pub const COVERAGE_BUFFERS_NV: c_uint = 0x30E0;
    pub const COVERAGE_SAMPLES_NV: c_uint = 0x30E1;
    pub const COVERAGE_SAMPLE_RESOLVE_DEFAULT_NV: c_uint = 0x3132;
    pub const COVERAGE_SAMPLE_RESOLVE_NONE_NV: c_uint = 0x3133;
    pub const COVERAGE_SAMPLE_RESOLVE_NV: c_uint = 0x3131;
    pub const CTA861_3_MAX_CONTENT_LIGHT_LEVEL_EXT: c_uint = 0x3360;
    pub const CTA861_3_MAX_FRAME_AVERAGE_LEVEL_EXT: c_uint = 0x3361;
    pub const CUDA_DEVICE_NV: c_uint = 0x323A;
    pub const CUDA_EVENT_HANDLE_NV: c_uint = 0x323B;
    pub const D3D11_DEVICE_ANGLE: c_uint = 0x33A1;
    pub const D3D9_DEVICE_ANGLE: c_uint = 0x33A0;
    pub const D3D_TEXTURE_2D_SHARE_HANDLE_ANGLE: c_uint = 0x3200;
    pub const DEBUG_CALLBACK_KHR: c_uint = 0x33B8;
    pub const DEBUG_MSG_CRITICAL_KHR: c_uint = 0x33B9;
    pub const DEBUG_MSG_ERROR_KHR: c_uint = 0x33BA;
    pub const DEBUG_MSG_INFO_KHR: c_uint = 0x33BC;
    pub const DEBUG_MSG_WARN_KHR: c_uint = 0x33BB;
    pub const DEFAULT_DISPLAY: EGLNativeDisplayType = 0 as EGLNativeDisplayType;
    pub const DEPTH_ENCODING_NONE_NV: c_uint = 0;
    pub const DEPTH_ENCODING_NONLINEAR_NV: c_uint = 0x30E3;
    pub const DEPTH_ENCODING_NV: c_uint = 0x30E2;
    pub const DEPTH_SIZE: c_uint = 0x3025;
    pub const DEQUEUE_READY_TIME_ANDROID: c_uint = 0x343B;
    pub const DEVICE_EXT: c_uint = 0x322C;
    pub const DEVICE_UUID_EXT: c_uint = 0x335C;
    pub const DISCARD_SAMPLES_ARM: c_uint = 0x3286;
    pub const DISPLAY_PRESENT_TIME_ANDROID: c_uint = 0x343A;
    pub const DISPLAY_SCALING: c_uint = 10000;
    pub const DMA_BUF_PLANE0_FD_EXT: c_uint = 0x3272;
    pub const DMA_BUF_PLANE0_MODIFIER_HI_EXT: c_uint = 0x3444;
    pub const DMA_BUF_PLANE0_MODIFIER_LO_EXT: c_uint = 0x3443;
    pub const DMA_BUF_PLANE0_OFFSET_EXT: c_uint = 0x3273;
    pub const DMA_BUF_PLANE0_PITCH_EXT: c_uint = 0x3274;
    pub const DMA_BUF_PLANE1_FD_EXT: c_uint = 0x3275;
    pub const DMA_BUF_PLANE1_MODIFIER_HI_EXT: c_uint = 0x3446;
    pub const DMA_BUF_PLANE1_MODIFIER_LO_EXT: c_uint = 0x3445;
    pub const DMA_BUF_PLANE1_OFFSET_EXT: c_uint = 0x3276;
    pub const DMA_BUF_PLANE1_PITCH_EXT: c_uint = 0x3277;
    pub const DMA_BUF_PLANE2_FD_EXT: c_uint = 0x3278;
    pub const DMA_BUF_PLANE2_MODIFIER_HI_EXT: c_uint = 0x3448;
    pub const DMA_BUF_PLANE2_MODIFIER_LO_EXT: c_uint = 0x3447;
    pub const DMA_BUF_PLANE2_OFFSET_EXT: c_uint = 0x3279;
    pub const DMA_BUF_PLANE2_PITCH_EXT: c_uint = 0x327A;
    pub const DMA_BUF_PLANE3_FD_EXT: c_uint = 0x3440;
    pub const DMA_BUF_PLANE3_MODIFIER_HI_EXT: c_uint = 0x344A;
    pub const DMA_BUF_PLANE3_MODIFIER_LO_EXT: c_uint = 0x3449;
    pub const DMA_BUF_PLANE3_OFFSET_EXT: c_uint = 0x3441;
    pub const DMA_BUF_PLANE3_PITCH_EXT: c_uint = 0x3442;
    pub const DONT_CARE: EGLint = -1 as EGLint;
    pub const DRAW: c_uint = 0x3059;
    pub const DRIVER_NAME_EXT: c_uint = 0x335E;
    pub const DRIVER_UUID_EXT: c_uint = 0x335D;
    pub const DRM_BUFFER_FORMAT_ARGB32_MESA: c_uint = 0x31D2;
    pub const DRM_BUFFER_FORMAT_MESA: c_uint = 0x31D0;
    pub const DRM_BUFFER_MESA: c_uint = 0x31D3;
    pub const DRM_BUFFER_STRIDE_MESA: c_uint = 0x31D4;
    pub const DRM_BUFFER_USE_CURSOR_MESA: c_uint = 0x00000004;
    pub const DRM_BUFFER_USE_MESA: c_uint = 0x31D1;
    pub const DRM_BUFFER_USE_SCANOUT_MESA: c_uint = 0x00000001;
    pub const DRM_BUFFER_USE_SHARE_MESA: c_uint = 0x00000002;
    pub const DRM_CONNECTOR_EXT: c_uint = 0x3236;
    pub const DRM_CRTC_EXT: c_uint = 0x3234;
    pub const DRM_DEVICE_FILE_EXT: c_uint = 0x3233;
    pub const DRM_MASTER_FD_EXT: c_uint = 0x333C;
    pub const DRM_PLANE_EXT: c_uint = 0x3235;
    pub const DRM_RENDER_NODE_FILE_EXT: c_uint = 0x3377;
    pub const EXTENSIONS: c_uint = 0x3055;
    pub const EXTERNAL_REF_ID_EXT: c_uint = 0x3461;
    pub const FALSE: c_uint = 0;
    pub const FIRST_COMPOSITION_GPU_FINISHED_TIME_ANDROID: c_uint = 0x3439;
    pub const FIRST_COMPOSITION_START_TIME_ANDROID: c_uint = 0x3437;
    pub const FIXED_SIZE_ANGLE: c_uint = 0x3201;
    pub const FOREVER: u64 = 0xFFFFFFFFFFFFFFFF;
    pub const FOREVER_KHR: u64 = 0xFFFFFFFFFFFFFFFF;
    pub const FOREVER_NV: u64 = 0xFFFFFFFFFFFFFFFF;
    pub const FORMAT_RGBA_8888_EXACT_KHR: c_uint = 0x30C2;
    pub const FORMAT_RGBA_8888_KHR: c_uint = 0x30C3;
    pub const FORMAT_RGB_565_EXACT_KHR: c_uint = 0x30C0;
    pub const FORMAT_RGB_565_KHR: c_uint = 0x30C1;
    pub const FRAMEBUFFER_TARGET_ANDROID: c_uint = 0x3147;
    pub const FRONT_BUFFER_AUTO_REFRESH_ANDROID: c_uint = 0x314C;
    pub const FRONT_BUFFER_EXT: c_uint = 0x3464;
    pub const GENERATE_RESET_ON_VIDEO_MEMORY_PURGE_NV: c_uint = 0x334C;
    pub const GL_COLORSPACE: c_uint = 0x309D;
    pub const GL_COLORSPACE_BT2020_LINEAR_EXT: c_uint = 0x333F;
    pub const GL_COLORSPACE_BT2020_PQ_EXT: c_uint = 0x3340;
    pub const GL_COLORSPACE_DEFAULT_EXT: c_uint = 0x314D;
    pub const GL_COLORSPACE_DISPLAY_P3_EXT: c_uint = 0x3363;
    pub const GL_COLORSPACE_DISPLAY_P3_LINEAR_EXT: c_uint = 0x3362;
    pub const GL_COLORSPACE_DISPLAY_P3_PASSTHROUGH_EXT: c_uint = 0x3490;
    pub const GL_COLORSPACE_KHR: c_uint = 0x309D;
    pub const GL_COLORSPACE_LINEAR: c_uint = 0x308A;
    pub const GL_COLORSPACE_LINEAR_KHR: c_uint = 0x308A;
    pub const GL_COLORSPACE_SCRGB_EXT: c_uint = 0x3351;
    pub const GL_COLORSPACE_SCRGB_LINEAR_EXT: c_uint = 0x3350;
    pub const GL_COLORSPACE_SRGB: c_uint = 0x3089;
    pub const GL_COLORSPACE_SRGB_KHR: c_uint = 0x3089;
    pub const GL_RENDERBUFFER: c_uint = 0x30B9;
    pub const GL_RENDERBUFFER_KHR: c_uint = 0x30B9;
    pub const GL_TEXTURE_2D: c_uint = 0x30B1;
    pub const GL_TEXTURE_2D_KHR: c_uint = 0x30B1;
    pub const GL_TEXTURE_3D: c_uint = 0x30B2;
    pub const GL_TEXTURE_3D_KHR: c_uint = 0x30B2;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: c_uint = 0x30B4;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X_KHR: c_uint = 0x30B4;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: c_uint = 0x30B6;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y_KHR: c_uint = 0x30B6;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: c_uint = 0x30B8;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z_KHR: c_uint = 0x30B8;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: c_uint = 0x30B3;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X_KHR: c_uint = 0x30B3;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: c_uint = 0x30B5;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y_KHR: c_uint = 0x30B5;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: c_uint = 0x30B7;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z_KHR: c_uint = 0x30B7;
    pub const GL_TEXTURE_LEVEL: c_uint = 0x30BC;
    pub const GL_TEXTURE_LEVEL_KHR: c_uint = 0x30BC;
    pub const GL_TEXTURE_ZOFFSET: c_uint = 0x30BD;
    pub const GL_TEXTURE_ZOFFSET_KHR: c_uint = 0x30BD;
    pub const GREEN_SIZE: c_uint = 0x3023;
    pub const HEIGHT: c_uint = 0x3056;
    pub const HORIZONTAL_RESOLUTION: c_uint = 0x3090;
    pub const IMAGE_PRESERVED: c_uint = 0x30D2;
    pub const IMAGE_PRESERVED_KHR: c_uint = 0x30D2;
    pub const IMPORT_EXPLICIT_SYNC_EXT: c_uint = 0x3472;
    pub const IMPORT_IMPLICIT_SYNC_EXT: c_uint = 0x3471;
    pub const IMPORT_SYNC_TYPE_EXT: c_uint = 0x3470;
    pub const ITU_REC2020_EXT: c_uint = 0x3281;
    pub const ITU_REC601_EXT: c_uint = 0x327F;
    pub const ITU_REC709_EXT: c_uint = 0x3280;
    pub const LARGEST_PBUFFER: c_uint = 0x3058;
    pub const LAST_COMPOSITION_START_TIME_ANDROID: c_uint = 0x3438;
    pub const LEFT_NV: c_uint = 0x336B;
    pub const LEVEL: c_uint = 0x3029;
    pub const LINUX_DMA_BUF_EXT: c_uint = 0x3270;
    pub const LINUX_DRM_FOURCC_EXT: c_uint = 0x3271;
    pub const LOCK_SURFACE_BIT_KHR: c_uint = 0x0080;
    pub const LOCK_USAGE_HINT_KHR: c_uint = 0x30C5;
    pub const LOSE_CONTEXT_ON_RESET: c_uint = 0x31BF;
    pub const LOSE_CONTEXT_ON_RESET_EXT: c_uint = 0x31BF;
    pub const LOSE_CONTEXT_ON_RESET_KHR: c_uint = 0x31BF;
    pub const LOWER_LEFT_KHR: c_uint = 0x30CE;
    pub const LUMINANCE_BUFFER: c_uint = 0x308F;
    pub const LUMINANCE_SIZE: c_uint = 0x303D;
    pub const MAP_PRESERVE_PIXELS_KHR: c_uint = 0x30C4;
    pub const MATCH_FORMAT_KHR: c_uint = 0x3043;
    pub const MATCH_NATIVE_PIXMAP: c_uint = 0x3041;
    pub const MAX_PBUFFER_HEIGHT: c_uint = 0x302A;
    pub const MAX_PBUFFER_PIXELS: c_uint = 0x302B;
    pub const MAX_PBUFFER_WIDTH: c_uint = 0x302C;
    pub const MAX_STREAM_METADATA_BLOCKS_NV: c_uint = 0x3250;
    pub const MAX_STREAM_METADATA_BLOCK_SIZE_NV: c_uint = 0x3251;
    pub const MAX_STREAM_METADATA_TOTAL_SIZE_NV: c_uint = 0x3252;
    pub const MAX_SWAP_INTERVAL: c_uint = 0x303C;
    pub const METADATA0_SIZE_NV: c_uint = 0x3255;
    pub const METADATA0_TYPE_NV: c_uint = 0x3259;
    pub const METADATA1_SIZE_NV: c_uint = 0x3256;
    pub const METADATA1_TYPE_NV: c_uint = 0x325A;
    pub const METADATA2_SIZE_NV: c_uint = 0x3257;
    pub const METADATA2_TYPE_NV: c_uint = 0x325B;
    pub const METADATA3_SIZE_NV: c_uint = 0x3258;
    pub const METADATA3_TYPE_NV: c_uint = 0x325C;
    pub const METADATA_SCALING_EXT: c_uint = 50000;
    pub const MIN_SWAP_INTERVAL: c_uint = 0x303B;
    pub const MIPMAP_LEVEL: c_uint = 0x3083;
    pub const MIPMAP_TEXTURE: c_uint = 0x3082;
    pub const MULTISAMPLE_RESOLVE: c_uint = 0x3099;
    pub const MULTISAMPLE_RESOLVE_BOX: c_uint = 0x309B;
    pub const MULTISAMPLE_RESOLVE_BOX_BIT: c_uint = 0x0200;
    pub const MULTISAMPLE_RESOLVE_DEFAULT: c_uint = 0x309A;
    pub const MULTIVIEW_VIEW_COUNT_EXT: c_uint = 0x3134;
    pub const MUTABLE_RENDER_BUFFER_BIT_KHR: c_uint = 0x1000;
    pub const NATIVE_BUFFER_ANDROID: c_uint = 0x3140;
    pub const NATIVE_BUFFER_MULTIPLANE_SEPARATE_IMG: c_uint = 0x3105;
    pub const NATIVE_BUFFER_PLANE_OFFSET_IMG: c_uint = 0x3106;
    pub const NATIVE_BUFFER_TIZEN: c_uint = 0x32A0;
    pub const NATIVE_BUFFER_USAGE_ANDROID: c_uint = 0x3143;
    pub const NATIVE_BUFFER_USAGE_PROTECTED_BIT_ANDROID: c_uint = 0x00000001;
    pub const NATIVE_BUFFER_USAGE_RENDERBUFFER_BIT_ANDROID: c_uint = 0x00000002;
    pub const NATIVE_BUFFER_USAGE_TEXTURE_BIT_ANDROID: c_uint = 0x00000004;
    pub const NATIVE_PIXMAP_KHR: c_uint = 0x30B0;
    pub const NATIVE_RENDERABLE: c_uint = 0x302D;
    pub const NATIVE_SURFACE_TIZEN: c_uint = 0x32A1;
    pub const NATIVE_VISUAL_ID: c_uint = 0x302E;
    pub const NATIVE_VISUAL_TYPE: c_uint = 0x302F;
    pub const NONE: c_uint = 0x3038;
    pub const NON_CONFORMANT_CONFIG: c_uint = 0x3051;
    pub const NOT_INITIALIZED: c_uint = 0x3001;
    pub const NO_CONFIG_KHR: EGLConfig = 0 as EGLConfig;
    pub const NO_CONTEXT: EGLContext = 0 as EGLContext;
    pub const NO_DEVICE_EXT: EGLDeviceEXT = 0 as EGLDeviceEXT;
    pub const NO_DISPLAY: EGLDisplay = 0 as EGLDisplay;
    pub const NO_FILE_DESCRIPTOR_KHR: EGLNativeFileDescriptorKHR = -1 as EGLNativeFileDescriptorKHR;
    pub const NO_IMAGE: EGLImage = 0 as EGLImage;
    pub const NO_IMAGE_KHR: EGLImageKHR = 0 as EGLImageKHR;
    pub const NO_NATIVE_FENCE_FD_ANDROID: c_int = -1;
    pub const NO_OUTPUT_LAYER_EXT: EGLOutputLayerEXT = 0 as EGLOutputLayerEXT;
    pub const NO_OUTPUT_PORT_EXT: EGLOutputPortEXT = 0 as EGLOutputPortEXT;
    pub const NO_RESET_NOTIFICATION: c_uint = 0x31BE;
    pub const NO_RESET_NOTIFICATION_EXT: c_uint = 0x31BE;
    pub const NO_RESET_NOTIFICATION_KHR: c_uint = 0x31BE;
    pub const NO_STREAM_KHR: EGLStreamKHR = 0 as EGLStreamKHR;
    pub const NO_SURFACE: EGLSurface = 0 as EGLSurface;
    pub const NO_SYNC: EGLSync = 0 as EGLSync;
    pub const NO_SYNC_KHR: EGLSyncKHR = 0 as EGLSyncKHR;
    pub const NO_SYNC_NV: EGLSyncNV = 0 as EGLSyncNV;
    pub const NO_TEXTURE: c_uint = 0x305C;
    pub const OBJECT_CONTEXT_KHR: c_uint = 0x33B2;
    pub const OBJECT_DISPLAY_KHR: c_uint = 0x33B1;
    pub const OBJECT_IMAGE_KHR: c_uint = 0x33B4;
    pub const OBJECT_STREAM_KHR: c_uint = 0x33B6;
    pub const OBJECT_SURFACE_KHR: c_uint = 0x33B3;
    pub const OBJECT_SYNC_KHR: c_uint = 0x33B5;
    pub const OBJECT_THREAD_KHR: c_uint = 0x33B0;
    pub const OPENGL_API: c_uint = 0x30A2;
    pub const OPENGL_BIT: c_uint = 0x0008;
    pub const OPENGL_ES2_BIT: c_uint = 0x0004;
    pub const OPENGL_ES3_BIT: c_uint = 0x00000040;
    pub const OPENGL_ES3_BIT_KHR: c_uint = 0x00000040;
    pub const OPENGL_ES_API: c_uint = 0x30A0;
    pub const OPENGL_ES_BIT: c_uint = 0x0001;
    pub const OPENVG_API: c_uint = 0x30A1;
    pub const OPENVG_BIT: c_uint = 0x0002;
    pub const OPENVG_IMAGE: c_uint = 0x3096;
    pub const OPENWF_DEVICE_EXT: c_uint = 0x333D;
    pub const OPENWF_DEVICE_ID_EXT: c_uint = 0x3237;
    pub const OPENWF_PIPELINE_ID_EXT: c_uint = 0x3238;
    pub const OPENWF_PORT_ID_EXT: c_uint = 0x3239;
    pub const OPTIMAL_FORMAT_BIT_KHR: c_uint = 0x0100;
    pub const PBUFFER_BIT: c_uint = 0x0001;
    pub const PENDING_FRAME_NV: c_uint = 0x3329;
    pub const PENDING_METADATA_NV: c_uint = 0x3328;
    pub const PIXEL_ASPECT_RATIO: c_uint = 0x3092;
    pub const PIXMAP_BIT: c_uint = 0x0002;
    pub const PLATFORM_ANDROID_KHR: c_uint = 0x3141;
    pub const PLATFORM_DEVICE_EXT: c_uint = 0x313F;
    pub const PLATFORM_GBM_KHR: c_uint = 0x31D7;
    pub const PLATFORM_GBM_MESA: c_uint = 0x31D7;
    pub const PLATFORM_SURFACELESS_MESA: c_uint = 0x31DD;
    pub const PLATFORM_WAYLAND_EXT: c_uint = 0x31D8;
    pub const PLATFORM_WAYLAND_KHR: c_uint = 0x31D8;
    pub const PLATFORM_X11_EXT: c_uint = 0x31D5;
    pub const PLATFORM_X11_KHR: c_uint = 0x31D5;
    pub const PLATFORM_X11_SCREEN_EXT: c_uint = 0x31D6;
    pub const PLATFORM_X11_SCREEN_KHR: c_uint = 0x31D6;
    pub const PLATFORM_XCB_EXT: c_uint = 0x31DC;
    pub const PLATFORM_XCB_SCREEN_EXT: c_uint = 0x31DE;
    pub const POST_SUB_BUFFER_SUPPORTED_NV: c_uint = 0x30BE;
    pub const PRESENT_OPAQUE_EXT: c_uint = 0x31DF;
    pub const PRIMARY_COMPOSITOR_CONTEXT_EXT: c_uint = 0x3460;
    pub const PRODUCER_AUTO_ORIENTATION_NV: c_uint = 0x336A;
    pub const PRODUCER_FRAME_KHR: c_uint = 0x3212;
    pub const PRODUCER_MAX_FRAME_HINT_NV: c_uint = 0x3337;
    pub const PRODUCER_METADATA_NV: c_uint = 0x3253;
    pub const PROTECTED_CONTENT_EXT: c_uint = 0x32C0;
    pub const QUADRUPLE_BUFFER_NV: c_uint = 0x3231;
    pub const READ: c_uint = 0x305A;
    pub const READS_DONE_TIME_ANDROID: c_uint = 0x343C;
    pub const READ_SURFACE_BIT_KHR: c_uint = 0x0001;
    pub const RECORDABLE_ANDROID: c_uint = 0x3142;
    pub const RED_SIZE: c_uint = 0x3024;
    pub const RENDERABLE_TYPE: c_uint = 0x3040;
    pub const RENDERER_EXT: c_uint = 0x335F;
    pub const RENDERING_COMPLETE_TIME_ANDROID: c_uint = 0x3435;
    pub const RENDER_BUFFER: c_uint = 0x3086;
    pub const REQUESTED_PRESENT_TIME_ANDROID: c_uint = 0x3434;
    pub const RGB_BUFFER: c_uint = 0x308E;
    pub const RIGHT_NV: c_uint = 0x336C;
    pub const SAMPLES: c_uint = 0x3031;
    pub const SAMPLE_BUFFERS: c_uint = 0x3032;
    pub const SAMPLE_RANGE_HINT_EXT: c_uint = 0x327C;
    pub const SIGNALED: c_uint = 0x30F2;
    pub const SIGNALED_KHR: c_uint = 0x30F2;
    pub const SIGNALED_NV: c_uint = 0x30E8;
    pub const SINGLE_BUFFER: c_uint = 0x3085;
    pub const SLOW_CONFIG: c_uint = 0x3050;
    pub const SMPTE2086_DISPLAY_PRIMARY_BX_EXT: c_uint = 0x3345;
    pub const SMPTE2086_DISPLAY_PRIMARY_BY_EXT: c_uint = 0x3346;
    pub const SMPTE2086_DISPLAY_PRIMARY_GX_EXT: c_uint = 0x3343;
    pub const SMPTE2086_DISPLAY_PRIMARY_GY_EXT: c_uint = 0x3344;
    pub const SMPTE2086_DISPLAY_PRIMARY_RX_EXT: c_uint = 0x3341;
    pub const SMPTE2086_DISPLAY_PRIMARY_RY_EXT: c_uint = 0x3342;
    pub const SMPTE2086_MAX_LUMINANCE_EXT: c_uint = 0x3349;
    pub const SMPTE2086_MIN_LUMINANCE_EXT: c_uint = 0x334A;
    pub const SMPTE2086_WHITE_POINT_X_EXT: c_uint = 0x3347;
    pub const SMPTE2086_WHITE_POINT_Y_EXT: c_uint = 0x3348;
    pub const SOCKET_HANDLE_NV: c_uint = 0x324C;
    pub const SOCKET_TYPE_INET_NV: c_uint = 0x324F;
    pub const SOCKET_TYPE_NV: c_uint = 0x324D;
    pub const SOCKET_TYPE_UNIX_NV: c_uint = 0x324E;
    pub const STENCIL_SIZE: c_uint = 0x3026;
    pub const STREAM_BIT_KHR: c_uint = 0x0800;
    pub const STREAM_CONSUMER_IMAGE_NV: c_uint = 0x3373;
    pub const STREAM_CONSUMER_NV: c_uint = 0x3248;
    pub const STREAM_CROSS_DISPLAY_NV: c_uint = 0x334E;
    pub const STREAM_CROSS_OBJECT_NV: c_uint = 0x334D;
    pub const STREAM_CROSS_PARTITION_NV: c_uint = 0x323F;
    pub const STREAM_CROSS_PROCESS_NV: c_uint = 0x3245;
    pub const STREAM_CROSS_SYSTEM_NV: c_uint = 0x334F;
    pub const STREAM_DMA_NV: c_uint = 0x3371;
    pub const STREAM_DMA_SERVER_NV: c_uint = 0x3372;
    pub const STREAM_ENDPOINT_NV: c_uint = 0x3243;
    pub const STREAM_FIFO_LENGTH_KHR: c_uint = 0x31FC;
    pub const STREAM_FIFO_SYNCHRONOUS_NV: c_uint = 0x3336;
    pub const STREAM_FRAME_MAJOR_AXIS_NV: c_uint = 0x3368;
    pub const STREAM_FRAME_ORIGIN_X_NV: c_uint = 0x3366;
    pub const STREAM_FRAME_ORIGIN_Y_NV: c_uint = 0x3367;
    pub const STREAM_IMAGE_ADD_NV: c_uint = 0x3374;
    pub const STREAM_IMAGE_AVAILABLE_NV: c_uint = 0x3376;
    pub const STREAM_IMAGE_REMOVE_NV: c_uint = 0x3375;
    pub const STREAM_LOCAL_NV: c_uint = 0x3244;
    pub const STREAM_PRODUCER_NV: c_uint = 0x3247;
    pub const STREAM_PROTOCOL_FD_NV: c_uint = 0x3246;
    pub const STREAM_PROTOCOL_NV: c_uint = 0x3242;
    pub const STREAM_PROTOCOL_SOCKET_NV: c_uint = 0x324B;
    pub const STREAM_STATE_CONNECTING_KHR: c_uint = 0x3216;
    pub const STREAM_STATE_CREATED_KHR: c_uint = 0x3215;
    pub const STREAM_STATE_DISCONNECTED_KHR: c_uint = 0x321A;
    pub const STREAM_STATE_EMPTY_KHR: c_uint = 0x3217;
    pub const STREAM_STATE_INITIALIZING_NV: c_uint = 0x3240;
    pub const STREAM_STATE_KHR: c_uint = 0x3214;
    pub const STREAM_STATE_NEW_FRAME_AVAILABLE_KHR: c_uint = 0x3218;
    pub const STREAM_STATE_OLD_FRAME_AVAILABLE_KHR: c_uint = 0x3219;
    pub const STREAM_TIME_CONSUMER_KHR: c_uint = 0x31FE;
    pub const STREAM_TIME_NOW_KHR: c_uint = 0x31FD;
    pub const STREAM_TIME_PENDING_NV: c_uint = 0x332A;
    pub const STREAM_TIME_PRODUCER_KHR: c_uint = 0x31FF;
    pub const STREAM_TYPE_NV: c_uint = 0x3241;
    pub const SUCCESS: c_uint = 0x3000;
    pub const SUPPORT_RESET_NV: c_uint = 0x3334;
    pub const SUPPORT_REUSE_NV: c_uint = 0x3335;
    pub const SURFACE_COMPRESSION_EXT: c_uint = 0x34B0;
    pub const SURFACE_COMPRESSION_FIXED_RATE_10BPC_EXT: c_uint = 0x34BD;
    pub const SURFACE_COMPRESSION_FIXED_RATE_11BPC_EXT: c_uint = 0x34BE;
    pub const SURFACE_COMPRESSION_FIXED_RATE_12BPC_EXT: c_uint = 0x34BF;
    pub const SURFACE_COMPRESSION_FIXED_RATE_1BPC_EXT: c_uint = 0x34B4;
    pub const SURFACE_COMPRESSION_FIXED_RATE_2BPC_EXT: c_uint = 0x34B5;
    pub const SURFACE_COMPRESSION_FIXED_RATE_3BPC_EXT: c_uint = 0x34B6;
    pub const SURFACE_COMPRESSION_FIXED_RATE_4BPC_EXT: c_uint = 0x34B7;
    pub const SURFACE_COMPRESSION_FIXED_RATE_5BPC_EXT: c_uint = 0x34B8;
    pub const SURFACE_COMPRESSION_FIXED_RATE_6BPC_EXT: c_uint = 0x34B9;
    pub const SURFACE_COMPRESSION_FIXED_RATE_7BPC_EXT: c_uint = 0x34BA;
    pub const SURFACE_COMPRESSION_FIXED_RATE_8BPC_EXT: c_uint = 0x34BB;
    pub const SURFACE_COMPRESSION_FIXED_RATE_9BPC_EXT: c_uint = 0x34BC;
    pub const SURFACE_COMPRESSION_FIXED_RATE_DEFAULT_EXT: c_uint = 0x34B2;
    pub const SURFACE_COMPRESSION_FIXED_RATE_NONE_EXT: c_uint = 0x34B1;
    pub const SURFACE_COMPRESSION_PLANE1_EXT: c_uint = 0x328E;
    pub const SURFACE_COMPRESSION_PLANE2_EXT: c_uint = 0x328F;
    pub const SURFACE_TYPE: c_uint = 0x3033;
    pub const SWAP_BEHAVIOR: c_uint = 0x3093;
    pub const SWAP_BEHAVIOR_PRESERVED_BIT: c_uint = 0x0400;
    pub const SWAP_INTERVAL_EXT: c_uint = 0x322F;
    pub const SYNC_CLIENT_EXT: c_uint = 0x3364;
    pub const SYNC_CLIENT_SIGNAL_EXT: c_uint = 0x3365;
    pub const SYNC_CL_EVENT: c_uint = 0x30FE;
    pub const SYNC_CL_EVENT_COMPLETE: c_uint = 0x30FF;
    pub const SYNC_CL_EVENT_COMPLETE_KHR: c_uint = 0x30FF;
    pub const SYNC_CL_EVENT_KHR: c_uint = 0x30FE;
    pub const SYNC_CONDITION: c_uint = 0x30F8;
    pub const SYNC_CONDITION_KHR: c_uint = 0x30F8;
    pub const SYNC_CONDITION_NV: c_uint = 0x30EE;
    pub const SYNC_CUDA_EVENT_COMPLETE_NV: c_uint = 0x323D;
    pub const SYNC_CUDA_EVENT_NV: c_uint = 0x323C;
    pub const SYNC_FENCE: c_uint = 0x30F9;
    pub const SYNC_FENCE_KHR: c_uint = 0x30F9;
    pub const SYNC_FENCE_NV: c_uint = 0x30EF;
    pub const SYNC_FLUSH_COMMANDS_BIT: c_uint = 0x0001;
    pub const SYNC_FLUSH_COMMANDS_BIT_KHR: c_uint = 0x0001;
    pub const SYNC_FLUSH_COMMANDS_BIT_NV: c_uint = 0x0001;
    pub const SYNC_NATIVE_FENCE_ANDROID: c_uint = 0x3144;
    pub const SYNC_NATIVE_FENCE_FD_ANDROID: c_uint = 0x3145;
    pub const SYNC_NATIVE_FENCE_SIGNALED_ANDROID: c_uint = 0x3146;
    pub const SYNC_NEW_FRAME_NV: c_uint = 0x321F;
    pub const SYNC_PRIOR_COMMANDS_COMPLETE: c_uint = 0x30F0;
    pub const SYNC_PRIOR_COMMANDS_COMPLETE_KHR: c_uint = 0x30F0;
    pub const SYNC_PRIOR_COMMANDS_COMPLETE_NV: c_uint = 0x30E6;
    pub const SYNC_PRIOR_COMMANDS_IMPLICIT_EXTERNAL_ARM: c_uint = 0x328A;
    pub const SYNC_REUSABLE_KHR: c_uint = 0x30FA;
    pub const SYNC_STATUS: c_uint = 0x30F1;
    pub const SYNC_STATUS_KHR: c_uint = 0x30F1;
    pub const SYNC_STATUS_NV: c_uint = 0x30E7;
    pub const SYNC_TYPE: c_uint = 0x30F7;
    pub const SYNC_TYPE_KHR: c_uint = 0x30F7;
    pub const SYNC_TYPE_NV: c_uint = 0x30ED;
    pub const TEXTURE_2D: c_uint = 0x305F;
    pub const TEXTURE_EXTERNAL_WL: c_uint = 0x31DA;
    pub const TEXTURE_FORMAT: c_uint = 0x3080;
    pub const TEXTURE_RGB: c_uint = 0x305D;
    pub const TEXTURE_RGBA: c_uint = 0x305E;
    pub const TEXTURE_TARGET: c_uint = 0x3081;
    pub const TEXTURE_Y_UV_WL: c_uint = 0x31D8;
    pub const TEXTURE_Y_U_V_WL: c_uint = 0x31D7;
    pub const TEXTURE_Y_XUXV_WL: c_uint = 0x31D9;
    pub const TIMEOUT_EXPIRED: c_uint = 0x30F5;
    pub const TIMEOUT_EXPIRED_KHR: c_uint = 0x30F5;
    pub const TIMEOUT_EXPIRED_NV: c_uint = 0x30EB;
    pub const TIMESTAMPS_ANDROID: c_uint = 0x3430;
    pub const TIMESTAMP_INVALID_ANDROID: EGLnsecsANDROID = -1 as EGLnsecsANDROID;
    pub const TIMESTAMP_PENDING_ANDROID: EGLnsecsANDROID = -2 as EGLnsecsANDROID;
    pub const TOP_NV: c_uint = 0x336D;
    pub const TRACK_REFERENCES_KHR: c_uint = 0x3352;
    pub const TRANSPARENT_BLUE_VALUE: c_uint = 0x3035;
    pub const TRANSPARENT_GREEN_VALUE: c_uint = 0x3036;
    pub const TRANSPARENT_RED_VALUE: c_uint = 0x3037;
    pub const TRANSPARENT_RGB: c_uint = 0x3052;
    pub const TRANSPARENT_TYPE: c_uint = 0x3034;
    pub const TRIPLE_BUFFER_NV: c_uint = 0x3230;
    pub const TRUE: c_uint = 1;
    pub const UNKNOWN: EGLint = -1 as EGLint;
    pub const UNSIGNALED: c_uint = 0x30F3;
    pub const UNSIGNALED_KHR: c_uint = 0x30F3;
    pub const UNSIGNALED_NV: c_uint = 0x30E9;
    pub const UPPER_LEFT_KHR: c_uint = 0x30CF;
    pub const VENDOR: c_uint = 0x3053;
    pub const VERSION: c_uint = 0x3054;
    pub const VERTICAL_RESOLUTION: c_uint = 0x3091;
    pub const VG_ALPHA_FORMAT: c_uint = 0x3088;
    pub const VG_ALPHA_FORMAT_NONPRE: c_uint = 0x308B;
    pub const VG_ALPHA_FORMAT_PRE: c_uint = 0x308C;
    pub const VG_ALPHA_FORMAT_PRE_BIT: c_uint = 0x0040;
    pub const VG_ALPHA_FORMAT_PRE_BIT_KHR: c_uint = 0x0040;
    pub const VG_COLORSPACE: c_uint = 0x3087;
    pub const VG_COLORSPACE_LINEAR: c_uint = 0x308A;
    pub const VG_COLORSPACE_LINEAR_BIT: c_uint = 0x0020;
    pub const VG_COLORSPACE_LINEAR_BIT_KHR: c_uint = 0x0020;
    pub const VG_COLORSPACE_sRGB: c_uint = 0x3089;
    pub const VG_PARENT_IMAGE_KHR: c_uint = 0x30BA;
    pub const WAYLAND_BUFFER_WL: c_uint = 0x31D5;
    pub const WAYLAND_PLANE_WL: c_uint = 0x31D6;
    pub const WAYLAND_Y_INVERTED_WL: c_uint = 0x31DB;
    pub const WIDTH: c_uint = 0x3057;
    pub const WINDOW_BIT: c_uint = 0x0004;
    pub const WRITE_SURFACE_BIT_KHR: c_uint = 0x0002;
    pub const X_AXIS_NV: c_uint = 0x336F;
    pub const YUV_BUFFER_EXT: c_uint = 0x3300;
    pub const YUV_CHROMA_HORIZONTAL_SITING_HINT_EXT: c_uint = 0x327D;
    pub const YUV_CHROMA_SITING_0_5_EXT: c_uint = 0x3285;
    pub const YUV_CHROMA_SITING_0_EXT: c_uint = 0x3284;
    pub const YUV_CHROMA_VERTICAL_SITING_HINT_EXT: c_uint = 0x327E;
    pub const YUV_COLOR_SPACE_HINT_EXT: c_uint = 0x327B;
    pub const YUV_CSC_STANDARD_2020_EXT: c_uint = 0x330D;
    pub const YUV_CSC_STANDARD_601_EXT: c_uint = 0x330B;
    pub const YUV_CSC_STANDARD_709_EXT: c_uint = 0x330C;
    pub const YUV_CSC_STANDARD_EXT: c_uint = 0x330A;
    pub const YUV_DEPTH_RANGE_EXT: c_uint = 0x3317;
    pub const YUV_DEPTH_RANGE_FULL_EXT: c_uint = 0x3319;
    pub const YUV_DEPTH_RANGE_LIMITED_EXT: c_uint = 0x3318;
    pub const YUV_FULL_RANGE_EXT: c_uint = 0x3282;
    pub const YUV_NARROW_RANGE_EXT: c_uint = 0x3283;
    pub const YUV_NUMBER_OF_PLANES_EXT: c_uint = 0x3311;
    pub const YUV_ORDER_AYUV_EXT: c_uint = 0x3308;
    pub const YUV_ORDER_EXT: c_uint = 0x3301;
    pub const YUV_ORDER_UYVY_EXT: c_uint = 0x3305;
    pub const YUV_ORDER_VYUY_EXT: c_uint = 0x3307;
    pub const YUV_ORDER_YUV_EXT: c_uint = 0x3302;
    pub const YUV_ORDER_YUYV_EXT: c_uint = 0x3304;
    pub const YUV_ORDER_YVU_EXT: c_uint = 0x3303;
    pub const YUV_ORDER_YVYU_EXT: c_uint = 0x3306;
    pub const YUV_PLANE0_TEXTURE_UNIT_NV: c_uint = 0x332C;
    pub const YUV_PLANE1_TEXTURE_UNIT_NV: c_uint = 0x332D;
    pub const YUV_PLANE2_TEXTURE_UNIT_NV: c_uint = 0x332E;
    pub const YUV_PLANE_BPP_0_EXT: c_uint = 0x331B;
    pub const YUV_PLANE_BPP_10_EXT: c_uint = 0x331D;
    pub const YUV_PLANE_BPP_8_EXT: c_uint = 0x331C;
    pub const YUV_PLANE_BPP_EXT: c_uint = 0x331A;
    pub const YUV_SUBSAMPLE_4_2_0_EXT: c_uint = 0x3313;
    pub const YUV_SUBSAMPLE_4_2_2_EXT: c_uint = 0x3314;
    pub const YUV_SUBSAMPLE_4_4_4_EXT: c_uint = 0x3315;
    pub const YUV_SUBSAMPLE_EXT: c_uint = 0x3312;
    pub const Y_AXIS_NV: c_uint = 0x3370;
    pub const Y_INVERTED_NOK: c_uint = 0x307F;
}

pub mod functions {
    #![allow(non_snake_case, unused_variables, dead_code, unused_imports)]

    use std::mem::transmute;
    use std::os::raw::*;
    use super::*;
    use super::types::*;

    macro_rules! func {
        ($fun:ident, $ret:ty, $($name:ident: $typ:ty),*) => {
            #[inline] pub unsafe fn $fun(&self, $($name: $typ),*) -> $ret {
                transmute::<_, extern "system" fn($($typ),*) -> $ret>(self.$fun.ptr)($($name),*)
            }
        }
    }

    pub struct Egl {
         pub(super) BindAPI: FnPtr,
         pub(super) BindTexImage: FnPtr,
         pub(super) BindWaylandDisplayWL: FnPtr,
         pub(super) ChooseConfig: FnPtr,
         pub(super) ClientSignalSyncEXT: FnPtr,
         pub(super) ClientWaitSync: FnPtr,
         pub(super) ClientWaitSyncKHR: FnPtr,
         pub(super) ClientWaitSyncNV: FnPtr,
         pub(super) CompositorBindTexWindowEXT: FnPtr,
         pub(super) CompositorSetContextAttributesEXT: FnPtr,
         pub(super) CompositorSetContextListEXT: FnPtr,
         pub(super) CompositorSetSizeEXT: FnPtr,
         pub(super) CompositorSetWindowAttributesEXT: FnPtr,
         pub(super) CompositorSetWindowListEXT: FnPtr,
         pub(super) CompositorSwapPolicyEXT: FnPtr,
         pub(super) CopyBuffers: FnPtr,
         pub(super) CreateContext: FnPtr,
         pub(super) CreateDRMImageMESA: FnPtr,
         pub(super) CreateFenceSyncNV: FnPtr,
         pub(super) CreateImage: FnPtr,
         pub(super) CreateImageKHR: FnPtr,
         pub(super) CreateNativeClientBufferANDROID: FnPtr,
         pub(super) CreatePbufferFromClientBuffer: FnPtr,
         pub(super) CreatePbufferSurface: FnPtr,
         pub(super) CreatePixmapSurface: FnPtr,
         pub(super) CreatePixmapSurfaceHI: FnPtr,
         pub(super) CreatePlatformPixmapSurface: FnPtr,
         pub(super) CreatePlatformPixmapSurfaceEXT: FnPtr,
         pub(super) CreatePlatformWindowSurface: FnPtr,
         pub(super) CreatePlatformWindowSurfaceEXT: FnPtr,
         pub(super) CreateStreamAttribKHR: FnPtr,
         pub(super) CreateStreamFromFileDescriptorKHR: FnPtr,
         pub(super) CreateStreamKHR: FnPtr,
         pub(super) CreateStreamProducerSurfaceKHR: FnPtr,
         pub(super) CreateStreamSyncNV: FnPtr,
         pub(super) CreateSync: FnPtr,
         pub(super) CreateSync64KHR: FnPtr,
         pub(super) CreateSyncKHR: FnPtr,
         pub(super) CreateWaylandBufferFromImageWL: FnPtr,
         pub(super) CreateWindowSurface: FnPtr,
         pub(super) DebugMessageControlKHR: FnPtr,
         pub(super) DestroyContext: FnPtr,
         pub(super) DestroyImage: FnPtr,
         pub(super) DestroyImageKHR: FnPtr,
         pub(super) DestroyStreamKHR: FnPtr,
         pub(super) DestroySurface: FnPtr,
         pub(super) DestroySync: FnPtr,
         pub(super) DestroySyncKHR: FnPtr,
         pub(super) DestroySyncNV: FnPtr,
         pub(super) DupNativeFenceFDANDROID: FnPtr,
         pub(super) ExportDMABUFImageMESA: FnPtr,
         pub(super) ExportDMABUFImageQueryMESA: FnPtr,
         pub(super) ExportDRMImageMESA: FnPtr,
         pub(super) FenceNV: FnPtr,
         pub(super) GetCompositorTimingANDROID: FnPtr,
         pub(super) GetCompositorTimingSupportedANDROID: FnPtr,
         pub(super) GetConfigAttrib: FnPtr,
         pub(super) GetConfigs: FnPtr,
         pub(super) GetCurrentContext: FnPtr,
         pub(super) GetCurrentDisplay: FnPtr,
         pub(super) GetCurrentSurface: FnPtr,
         pub(super) GetDisplay: FnPtr,
         pub(super) GetDisplayDriverConfig: FnPtr,
         pub(super) GetDisplayDriverName: FnPtr,
         pub(super) GetError: FnPtr,
         pub(super) GetFrameTimestampSupportedANDROID: FnPtr,
         pub(super) GetFrameTimestampsANDROID: FnPtr,
         pub(super) GetMscRateANGLE: FnPtr,
         pub(super) GetNativeClientBufferANDROID: FnPtr,
         pub(super) GetNextFrameIdANDROID: FnPtr,
         pub(super) GetOutputLayersEXT: FnPtr,
         pub(super) GetOutputPortsEXT: FnPtr,
         pub(super) GetPlatformDisplay: FnPtr,
         pub(super) GetPlatformDisplayEXT: FnPtr,
         pub(super) GetProcAddress: FnPtr,
         pub(super) GetStreamFileDescriptorKHR: FnPtr,
         pub(super) GetSyncAttrib: FnPtr,
         pub(super) GetSyncAttribKHR: FnPtr,
         pub(super) GetSyncAttribNV: FnPtr,
         pub(super) GetSystemTimeFrequencyNV: FnPtr,
         pub(super) GetSystemTimeNV: FnPtr,
         pub(super) Initialize: FnPtr,
         pub(super) LabelObjectKHR: FnPtr,
         pub(super) LockSurfaceKHR: FnPtr,
         pub(super) MakeCurrent: FnPtr,
         pub(super) OutputLayerAttribEXT: FnPtr,
         pub(super) OutputPortAttribEXT: FnPtr,
         pub(super) PostSubBufferNV: FnPtr,
         pub(super) PresentationTimeANDROID: FnPtr,
         pub(super) QueryAPI: FnPtr,
         pub(super) QueryContext: FnPtr,
         pub(super) QueryDebugKHR: FnPtr,
         pub(super) QueryDeviceAttribEXT: FnPtr,
         pub(super) QueryDeviceBinaryEXT: FnPtr,
         pub(super) QueryDeviceStringEXT: FnPtr,
         pub(super) QueryDevicesEXT: FnPtr,
         pub(super) QueryDisplayAttribEXT: FnPtr,
         pub(super) QueryDisplayAttribKHR: FnPtr,
         pub(super) QueryDisplayAttribNV: FnPtr,
         pub(super) QueryDmaBufFormatsEXT: FnPtr,
         pub(super) QueryDmaBufModifiersEXT: FnPtr,
         pub(super) QueryNativeDisplayNV: FnPtr,
         pub(super) QueryNativePixmapNV: FnPtr,
         pub(super) QueryNativeWindowNV: FnPtr,
         pub(super) QueryOutputLayerAttribEXT: FnPtr,
         pub(super) QueryOutputLayerStringEXT: FnPtr,
         pub(super) QueryOutputPortAttribEXT: FnPtr,
         pub(super) QueryOutputPortStringEXT: FnPtr,
         pub(super) QueryStreamAttribKHR: FnPtr,
         pub(super) QueryStreamConsumerEventNV: FnPtr,
         pub(super) QueryStreamKHR: FnPtr,
         pub(super) QueryStreamMetadataNV: FnPtr,
         pub(super) QueryStreamTimeKHR: FnPtr,
         pub(super) QueryStreamu64KHR: FnPtr,
         pub(super) QueryString: FnPtr,
         pub(super) QuerySupportedCompressionRatesEXT: FnPtr,
         pub(super) QuerySurface: FnPtr,
         pub(super) QuerySurface64KHR: FnPtr,
         pub(super) QuerySurfacePointerANGLE: FnPtr,
         pub(super) QueryWaylandBufferWL: FnPtr,
         pub(super) ReleaseTexImage: FnPtr,
         pub(super) ReleaseThread: FnPtr,
         pub(super) ResetStreamNV: FnPtr,
         pub(super) SetBlobCacheFuncsANDROID: FnPtr,
         pub(super) SetDamageRegionKHR: FnPtr,
         pub(super) SetStreamAttribKHR: FnPtr,
         pub(super) SetStreamMetadataNV: FnPtr,
         pub(super) SignalSyncKHR: FnPtr,
         pub(super) SignalSyncNV: FnPtr,
         pub(super) StreamAcquireImageNV: FnPtr,
         pub(super) StreamAttribKHR: FnPtr,
         pub(super) StreamConsumerAcquireAttribKHR: FnPtr,
         pub(super) StreamConsumerAcquireKHR: FnPtr,
         pub(super) StreamConsumerGLTextureExternalAttribsNV: FnPtr,
         pub(super) StreamConsumerGLTextureExternalKHR: FnPtr,
         pub(super) StreamConsumerOutputEXT: FnPtr,
         pub(super) StreamConsumerReleaseAttribKHR: FnPtr,
         pub(super) StreamConsumerReleaseKHR: FnPtr,
         pub(super) StreamFlushNV: FnPtr,
         pub(super) StreamImageConsumerConnectNV: FnPtr,
         pub(super) StreamReleaseImageNV: FnPtr,
         pub(super) SurfaceAttrib: FnPtr,
         pub(super) SwapBuffers: FnPtr,
         pub(super) SwapBuffersRegion2NOK: FnPtr,
         pub(super) SwapBuffersRegionNOK: FnPtr,
         pub(super) SwapBuffersWithDamageEXT: FnPtr,
         pub(super) SwapBuffersWithDamageKHR: FnPtr,
         pub(super) SwapInterval: FnPtr,
         pub(super) Terminate: FnPtr,
         pub(super) UnbindWaylandDisplayWL: FnPtr,
         pub(super) UnlockSurfaceKHR: FnPtr,
         pub(super) UnsignalSyncEXT: FnPtr,
         pub(super) WaitClient: FnPtr,
         pub(super) WaitGL: FnPtr,
         pub(super) WaitNative: FnPtr,
         pub(super) WaitSync: FnPtr,
         pub(super) WaitSyncKHR: FnPtr,
    }


    impl Egl {

     func!(BindAPI, EGLBoolean, api: EGLenum);
     func!(BindTexImage, EGLBoolean, dpy: EGLDisplay, surface: EGLSurface, buffer: EGLint);
     func!(BindWaylandDisplayWL, EGLBoolean, dpy: EGLDisplay, display: *mut wl_display);
     func!(ChooseConfig, EGLBoolean, dpy: EGLDisplay, attrib_list: *const EGLint, configs: *mut EGLConfig, config_size: EGLint, num_config: *mut EGLint);
     func!(ClientSignalSyncEXT, EGLBoolean, dpy: EGLDisplay, sync: EGLSync, attrib_list: *const EGLAttrib);
     func!(ClientWaitSync, EGLint, dpy: EGLDisplay, sync: EGLSync, flags: EGLint, timeout: EGLTime);
     func!(ClientWaitSyncKHR, EGLint, dpy: EGLDisplay, sync: EGLSyncKHR, flags: EGLint, timeout: EGLTimeKHR);
     func!(ClientWaitSyncNV, EGLint, sync: EGLSyncNV, flags: EGLint, timeout: EGLTimeNV);
     func!(CompositorBindTexWindowEXT, EGLBoolean, external_win_id: EGLint);
     func!(CompositorSetContextAttributesEXT, EGLBoolean, external_ref_id: EGLint, context_attributes: *const EGLint, num_entries: EGLint);
     func!(CompositorSetContextListEXT, EGLBoolean, external_ref_ids: *const EGLint, num_entries: EGLint);
     func!(CompositorSetSizeEXT, EGLBoolean, external_win_id: EGLint, width: EGLint, height: EGLint);
     func!(CompositorSetWindowAttributesEXT, EGLBoolean, external_win_id: EGLint, window_attributes: *const EGLint, num_entries: EGLint);
     func!(CompositorSetWindowListEXT, EGLBoolean, external_ref_id: EGLint, external_win_ids: *const EGLint, num_entries: EGLint);
     func!(CompositorSwapPolicyEXT, EGLBoolean, external_win_id: EGLint, policy: EGLint);
     func!(CopyBuffers, EGLBoolean, dpy: EGLDisplay, surface: EGLSurface, target: EGLNativePixmapType);
     func!(CreateContext, EGLContext, dpy: EGLDisplay, config: EGLConfig, share_context: EGLContext, attrib_list: *const EGLint);
     func!(CreateDRMImageMESA, EGLImageKHR, dpy: EGLDisplay, attrib_list: *const EGLint);
     func!(CreateFenceSyncNV, EGLSyncNV, dpy: EGLDisplay, condition: EGLenum, attrib_list: *const EGLint);
     func!(CreateImage, EGLImage, dpy: EGLDisplay, ctx: EGLContext, target: EGLenum, buffer: EGLClientBuffer, attrib_list: *const EGLAttrib);
     func!(CreateImageKHR, EGLImageKHR, dpy: EGLDisplay, ctx: EGLContext, target: EGLenum, buffer: EGLClientBuffer, attrib_list: *const EGLint);
     func!(CreateNativeClientBufferANDROID, EGLClientBuffer, attrib_list: *const EGLint);
     func!(CreatePbufferFromClientBuffer, EGLSurface, dpy: EGLDisplay, buftype: EGLenum, buffer: EGLClientBuffer, config: EGLConfig, attrib_list: *const EGLint);
     func!(CreatePbufferSurface, EGLSurface, dpy: EGLDisplay, config: EGLConfig, attrib_list: *const EGLint);
     func!(CreatePixmapSurface, EGLSurface, dpy: EGLDisplay, config: EGLConfig, pixmap: EGLNativePixmapType, attrib_list: *const EGLint);
     func!(CreatePixmapSurfaceHI, EGLSurface, dpy: EGLDisplay, config: EGLConfig, pixmap: *mut EGLClientPixmapHI);
     func!(CreatePlatformPixmapSurface, EGLSurface, dpy: EGLDisplay, config: EGLConfig, native_pixmap: *mut c_void, attrib_list: *const EGLAttrib);
     func!(CreatePlatformPixmapSurfaceEXT, EGLSurface, dpy: EGLDisplay, config: EGLConfig, native_pixmap: *mut c_void, attrib_list: *const EGLint);
     func!(CreatePlatformWindowSurface, EGLSurface, dpy: EGLDisplay, config: EGLConfig, native_window: *mut c_void, attrib_list: *const EGLAttrib);
     func!(CreatePlatformWindowSurfaceEXT, EGLSurface, dpy: EGLDisplay, config: EGLConfig, native_window: *mut c_void, attrib_list: *const EGLint);
     func!(CreateStreamAttribKHR, EGLStreamKHR, dpy: EGLDisplay, attrib_list: *const EGLAttrib);
     func!(CreateStreamFromFileDescriptorKHR, EGLStreamKHR, dpy: EGLDisplay, file_descriptor: EGLNativeFileDescriptorKHR);
     func!(CreateStreamKHR, EGLStreamKHR, dpy: EGLDisplay, attrib_list: *const EGLint);
     func!(CreateStreamProducerSurfaceKHR, EGLSurface, dpy: EGLDisplay, config: EGLConfig, stream: EGLStreamKHR, attrib_list: *const EGLint);
     func!(CreateStreamSyncNV, EGLSyncKHR, dpy: EGLDisplay, stream: EGLStreamKHR, type_: EGLenum, attrib_list: *const EGLint);
     func!(CreateSync, EGLSync, dpy: EGLDisplay, type_: EGLenum, attrib_list: *const EGLAttrib);
     func!(CreateSync64KHR, EGLSyncKHR, dpy: EGLDisplay, type_: EGLenum, attrib_list: *const EGLAttribKHR);
     func!(CreateSyncKHR, EGLSyncKHR, dpy: EGLDisplay, type_: EGLenum, attrib_list: *const EGLint);
     func!(CreateWaylandBufferFromImageWL, *mut wl_buffer, dpy: EGLDisplay, image: EGLImageKHR);
     func!(CreateWindowSurface, EGLSurface, dpy: EGLDisplay, config: EGLConfig, win: EGLNativeWindowType, attrib_list: *const EGLint);
     func!(DebugMessageControlKHR, EGLint, callback: EGLDEBUGPROCKHR, attrib_list: *const EGLAttrib);
     func!(DestroyContext, EGLBoolean, dpy: EGLDisplay, ctx: EGLContext);
     func!(DestroyImage, EGLBoolean, dpy: EGLDisplay, image: EGLImage);
     func!(DestroyImageKHR, EGLBoolean, dpy: EGLDisplay, image: EGLImageKHR);
     func!(DestroyStreamKHR, EGLBoolean, dpy: EGLDisplay, stream: EGLStreamKHR);
     func!(DestroySurface, EGLBoolean, dpy: EGLDisplay, surface: EGLSurface);
     func!(DestroySync, EGLBoolean, dpy: EGLDisplay, sync: EGLSync);
     func!(DestroySyncKHR, EGLBoolean, dpy: EGLDisplay, sync: EGLSyncKHR);
     func!(DestroySyncNV, EGLBoolean, sync: EGLSyncNV);
     func!(DupNativeFenceFDANDROID, EGLint, dpy: EGLDisplay, sync: EGLSyncKHR);
     func!(ExportDMABUFImageMESA, EGLBoolean, dpy: EGLDisplay, image: EGLImageKHR, fds: *mut c_int, strides: *mut EGLint, offsets: *mut EGLint);
     func!(ExportDMABUFImageQueryMESA, EGLBoolean, dpy: EGLDisplay, image: EGLImageKHR, fourcc: *mut c_int, num_planes: *mut c_int, modifiers: *mut EGLuint64KHR);
     func!(ExportDRMImageMESA, EGLBoolean, dpy: EGLDisplay, image: EGLImageKHR, name: *mut EGLint, handle: *mut EGLint, stride: *mut EGLint);
     func!(FenceNV, EGLBoolean, sync: EGLSyncNV);
     func!(GetCompositorTimingANDROID, EGLBoolean, dpy: EGLDisplay, surface: EGLSurface, numTimestamps: EGLint, names: *const EGLint, values: *mut EGLnsecsANDROID);
     func!(GetCompositorTimingSupportedANDROID, EGLBoolean, dpy: EGLDisplay, surface: EGLSurface, name: EGLint);
     func!(GetConfigAttrib, EGLBoolean, dpy: EGLDisplay, config: EGLConfig, attribute: EGLint, value: *mut EGLint);
     func!(GetConfigs, EGLBoolean, dpy: EGLDisplay, configs: *mut EGLConfig, config_size: EGLint, num_config: *mut EGLint);
     func!(GetCurrentContext, EGLContext, );
     func!(GetCurrentDisplay, EGLDisplay, );
     func!(GetCurrentSurface, EGLSurface, readdraw: EGLint);
     func!(GetDisplay, EGLDisplay, display_id: EGLNativeDisplayType);
     func!(GetDisplayDriverConfig, *mut c_char, dpy: EGLDisplay);
     func!(GetDisplayDriverName, *const c_char, dpy: EGLDisplay);
     func!(GetError, EGLint, );
     func!(GetFrameTimestampSupportedANDROID, EGLBoolean, dpy: EGLDisplay, surface: EGLSurface, timestamp: EGLint);
     func!(GetFrameTimestampsANDROID, EGLBoolean, dpy: EGLDisplay, surface: EGLSurface, frameId: EGLuint64KHR, numTimestamps: EGLint, timestamps: *const EGLint, values: *mut EGLnsecsANDROID);
     func!(GetMscRateANGLE, EGLBoolean, dpy: EGLDisplay, surface: EGLSurface, numerator: *mut EGLint, denominator: *mut EGLint);
     func!(GetNativeClientBufferANDROID, EGLClientBuffer, buffer: *const AHardwareBuffer);
     func!(GetNextFrameIdANDROID, EGLBoolean, dpy: EGLDisplay, surface: EGLSurface, frameId: *mut EGLuint64KHR);
     func!(GetOutputLayersEXT, EGLBoolean, dpy: EGLDisplay, attrib_list: *const EGLAttrib, layers: *mut EGLOutputLayerEXT, max_layers: EGLint, num_layers: *mut EGLint);
     func!(GetOutputPortsEXT, EGLBoolean, dpy: EGLDisplay, attrib_list: *const EGLAttrib, ports: *mut EGLOutputPortEXT, max_ports: EGLint, num_ports: *mut EGLint);
     func!(GetPlatformDisplay, EGLDisplay, platform: EGLenum, native_display: *mut c_void, attrib_list: *const EGLAttrib);
     func!(GetPlatformDisplayEXT, EGLDisplay, platform: EGLenum, native_display: *mut c_void, attrib_list: *const EGLint);
     func!(GetProcAddress, __eglMustCastToProperFunctionPointerType, procname: *const c_char);
     func!(GetStreamFileDescriptorKHR, EGLNativeFileDescriptorKHR, dpy: EGLDisplay, stream: EGLStreamKHR);
     func!(GetSyncAttrib, EGLBoolean, dpy: EGLDisplay, sync: EGLSync, attribute: EGLint, value: *mut EGLAttrib);
     func!(GetSyncAttribKHR, EGLBoolean, dpy: EGLDisplay, sync: EGLSyncKHR, attribute: EGLint, value: *mut EGLint);
     func!(GetSyncAttribNV, EGLBoolean, sync: EGLSyncNV, attribute: EGLint, value: *mut EGLint);
     func!(GetSystemTimeFrequencyNV, EGLuint64NV, );
     func!(GetSystemTimeNV, EGLuint64NV, );
     func!(Initialize, EGLBoolean, dpy: EGLDisplay, major: *mut EGLint, minor: *mut EGLint);
     func!(LabelObjectKHR, EGLint, display: EGLDisplay, objectType: EGLenum, object: EGLObjectKHR, label: EGLLabelKHR);
     func!(LockSurfaceKHR, EGLBoolean, dpy: EGLDisplay, surface: EGLSurface, attrib_list: *const EGLint);
     func!(MakeCurrent, EGLBoolean, dpy: EGLDisplay, draw: EGLSurface, read: EGLSurface, ctx: EGLContext);
     func!(OutputLayerAttribEXT, EGLBoolean, dpy: EGLDisplay, layer: EGLOutputLayerEXT, attribute: EGLint, value: EGLAttrib);
     func!(OutputPortAttribEXT, EGLBoolean, dpy: EGLDisplay, port: EGLOutputPortEXT, attribute: EGLint, value: EGLAttrib);
     func!(PostSubBufferNV, EGLBoolean, dpy: EGLDisplay, surface: EGLSurface, x: EGLint, y: EGLint, width: EGLint, height: EGLint);
     func!(PresentationTimeANDROID, EGLBoolean, dpy: EGLDisplay, surface: EGLSurface, time: EGLnsecsANDROID);
     func!(QueryAPI, EGLenum, );
     func!(QueryContext, EGLBoolean, dpy: EGLDisplay, ctx: EGLContext, attribute: EGLint, value: *mut EGLint);
     func!(QueryDebugKHR, EGLBoolean, attribute: EGLint, value: *mut EGLAttrib);
     func!(QueryDeviceAttribEXT, EGLBoolean, device: EGLDeviceEXT, attribute: EGLint, value: *mut EGLAttrib);
     func!(QueryDeviceBinaryEXT, EGLBoolean, device: EGLDeviceEXT, name: EGLint, max_size: EGLint, value: *mut c_void, size: *mut EGLint);
     func!(QueryDeviceStringEXT, *const c_char, device: EGLDeviceEXT, name: EGLint);
     func!(QueryDevicesEXT, EGLBoolean, max_devices: EGLint, devices: *mut EGLDeviceEXT, num_devices: *mut EGLint);
     func!(QueryDisplayAttribEXT, EGLBoolean, dpy: EGLDisplay, attribute: EGLint, value: *mut EGLAttrib);
     func!(QueryDisplayAttribKHR, EGLBoolean, dpy: EGLDisplay, name: EGLint, value: *mut EGLAttrib);
     func!(QueryDisplayAttribNV, EGLBoolean, dpy: EGLDisplay, attribute: EGLint, value: *mut EGLAttrib);
     func!(QueryDmaBufFormatsEXT, EGLBoolean, dpy: EGLDisplay, max_formats: EGLint, formats: *mut EGLint, num_formats: *mut EGLint);
     func!(QueryDmaBufModifiersEXT, EGLBoolean, dpy: EGLDisplay, format: EGLint, max_modifiers: EGLint, modifiers: *mut EGLuint64KHR, external_only: *mut EGLBoolean, num_modifiers: *mut EGLint);
     func!(QueryNativeDisplayNV, EGLBoolean, dpy: EGLDisplay, display_id: *mut EGLNativeDisplayType);
     func!(QueryNativePixmapNV, EGLBoolean, dpy: EGLDisplay, surf: EGLSurface, pixmap: *mut EGLNativePixmapType);
     func!(QueryNativeWindowNV, EGLBoolean, dpy: EGLDisplay, surf: EGLSurface, window: *mut EGLNativeWindowType);
     func!(QueryOutputLayerAttribEXT, EGLBoolean, dpy: EGLDisplay, layer: EGLOutputLayerEXT, attribute: EGLint, value: *mut EGLAttrib);
     func!(QueryOutputLayerStringEXT, *const c_char, dpy: EGLDisplay, layer: EGLOutputLayerEXT, name: EGLint);
     func!(QueryOutputPortAttribEXT, EGLBoolean, dpy: EGLDisplay, port: EGLOutputPortEXT, attribute: EGLint, value: *mut EGLAttrib);
     func!(QueryOutputPortStringEXT, *const c_char, dpy: EGLDisplay, port: EGLOutputPortEXT, name: EGLint);
     func!(QueryStreamAttribKHR, EGLBoolean, dpy: EGLDisplay, stream: EGLStreamKHR, attribute: EGLenum, value: *mut EGLAttrib);
     func!(QueryStreamConsumerEventNV, EGLint, dpy: EGLDisplay, stream: EGLStreamKHR, timeout: EGLTime, event: *mut EGLenum, aux: *mut EGLAttrib);
     func!(QueryStreamKHR, EGLBoolean, dpy: EGLDisplay, stream: EGLStreamKHR, attribute: EGLenum, value: *mut EGLint);
     func!(QueryStreamMetadataNV, EGLBoolean, dpy: EGLDisplay, stream: EGLStreamKHR, name: EGLenum, n: EGLint, offset: EGLint, size: EGLint, data: *mut c_void);
     func!(QueryStreamTimeKHR, EGLBoolean, dpy: EGLDisplay, stream: EGLStreamKHR, attribute: EGLenum, value: *mut EGLTimeKHR);
     func!(QueryStreamu64KHR, EGLBoolean, dpy: EGLDisplay, stream: EGLStreamKHR, attribute: EGLenum, value: *mut EGLuint64KHR);
     func!(QueryString, *const c_char, dpy: EGLDisplay, name: EGLint);
     func!(QuerySupportedCompressionRatesEXT, EGLBoolean, dpy: EGLDisplay, config: EGLConfig, attrib_list: *const EGLAttrib, rates: *mut EGLint, rate_size: EGLint, num_rates: *mut EGLint);
     func!(QuerySurface, EGLBoolean, dpy: EGLDisplay, surface: EGLSurface, attribute: EGLint, value: *mut EGLint);
     func!(QuerySurface64KHR, EGLBoolean, dpy: EGLDisplay, surface: EGLSurface, attribute: EGLint, value: *mut EGLAttribKHR);
     func!(QuerySurfacePointerANGLE, EGLBoolean, dpy: EGLDisplay, surface: EGLSurface, attribute: EGLint, value: *mut *mut c_void);
     func!(QueryWaylandBufferWL, EGLBoolean, dpy: EGLDisplay, buffer: *mut wl_resource, attribute: EGLint, value: *mut EGLint);
     func!(ReleaseTexImage, EGLBoolean, dpy: EGLDisplay, surface: EGLSurface, buffer: EGLint);
     func!(ReleaseThread, EGLBoolean, );
     func!(ResetStreamNV, EGLBoolean, dpy: EGLDisplay, stream: EGLStreamKHR);
     func!(SetBlobCacheFuncsANDROID, (), dpy: EGLDisplay, set: EGLSetBlobFuncANDROID, get: EGLGetBlobFuncANDROID);
     func!(SetDamageRegionKHR, EGLBoolean, dpy: EGLDisplay, surface: EGLSurface, rects: *mut EGLint, n_rects: EGLint);
     func!(SetStreamAttribKHR, EGLBoolean, dpy: EGLDisplay, stream: EGLStreamKHR, attribute: EGLenum, value: EGLAttrib);
     func!(SetStreamMetadataNV, EGLBoolean, dpy: EGLDisplay, stream: EGLStreamKHR, n: EGLint, offset: EGLint, size: EGLint, data: *const c_void);
     func!(SignalSyncKHR, EGLBoolean, dpy: EGLDisplay, sync: EGLSyncKHR, mode: EGLenum);
     func!(SignalSyncNV, EGLBoolean, sync: EGLSyncNV, mode: EGLenum);
     func!(StreamAcquireImageNV, EGLBoolean, dpy: EGLDisplay, stream: EGLStreamKHR, pImage: *mut EGLImage, sync: EGLSync);
     func!(StreamAttribKHR, EGLBoolean, dpy: EGLDisplay, stream: EGLStreamKHR, attribute: EGLenum, value: EGLint);
     func!(StreamConsumerAcquireAttribKHR, EGLBoolean, dpy: EGLDisplay, stream: EGLStreamKHR, attrib_list: *const EGLAttrib);
     func!(StreamConsumerAcquireKHR, EGLBoolean, dpy: EGLDisplay, stream: EGLStreamKHR);
     func!(StreamConsumerGLTextureExternalAttribsNV, EGLBoolean, dpy: EGLDisplay, stream: EGLStreamKHR, attrib_list: *const EGLAttrib);
     func!(StreamConsumerGLTextureExternalKHR, EGLBoolean, dpy: EGLDisplay, stream: EGLStreamKHR);
     func!(StreamConsumerOutputEXT, EGLBoolean, dpy: EGLDisplay, stream: EGLStreamKHR, layer: EGLOutputLayerEXT);
     func!(StreamConsumerReleaseAttribKHR, EGLBoolean, dpy: EGLDisplay, stream: EGLStreamKHR, attrib_list: *const EGLAttrib);
     func!(StreamConsumerReleaseKHR, EGLBoolean, dpy: EGLDisplay, stream: EGLStreamKHR);
     func!(StreamFlushNV, EGLBoolean, dpy: EGLDisplay, stream: EGLStreamKHR);
     func!(StreamImageConsumerConnectNV, EGLBoolean, dpy: EGLDisplay, stream: EGLStreamKHR, num_modifiers: EGLint, modifiers: *const EGLuint64KHR, attrib_list: *const EGLAttrib);
     func!(StreamReleaseImageNV, EGLBoolean, dpy: EGLDisplay, stream: EGLStreamKHR, image: EGLImage, sync: EGLSync);
     func!(SurfaceAttrib, EGLBoolean, dpy: EGLDisplay, surface: EGLSurface, attribute: EGLint, value: EGLint);
     func!(SwapBuffers, EGLBoolean, dpy: EGLDisplay, surface: EGLSurface);
     func!(SwapBuffersRegion2NOK, EGLBoolean, dpy: EGLDisplay, surface: EGLSurface, numRects: EGLint, rects: *const EGLint);
     func!(SwapBuffersRegionNOK, EGLBoolean, dpy: EGLDisplay, surface: EGLSurface, numRects: EGLint, rects: *const EGLint);
     func!(SwapBuffersWithDamageEXT, EGLBoolean, dpy: EGLDisplay, surface: EGLSurface, rects: *const EGLint, n_rects: EGLint);
     func!(SwapBuffersWithDamageKHR, EGLBoolean, dpy: EGLDisplay, surface: EGLSurface, rects: *const EGLint, n_rects: EGLint);
     func!(SwapInterval, EGLBoolean, dpy: EGLDisplay, interval: EGLint);
     func!(Terminate, EGLBoolean, dpy: EGLDisplay);
     func!(UnbindWaylandDisplayWL, EGLBoolean, dpy: EGLDisplay, display: *mut wl_display);
     func!(UnlockSurfaceKHR, EGLBoolean, dpy: EGLDisplay, surface: EGLSurface);
     func!(UnsignalSyncEXT, EGLBoolean, dpy: EGLDisplay, sync: EGLSync, attrib_list: *const EGLAttrib);
     func!(WaitClient, EGLBoolean, );
     func!(WaitGL, EGLBoolean, );
     func!(WaitNative, EGLBoolean, engine: EGLint);
     func!(WaitSync, EGLBoolean, dpy: EGLDisplay, sync: EGLSync, flags: EGLint);
     func!(WaitSyncKHR, EGLint, dpy: EGLDisplay, sync: EGLSyncKHR, flags: EGLint);

    }
}


pub fn load<F>(mut loadfn: F) -> functions::Egl where F: FnMut(&'static str) -> *const c_void {
    #[allow(unused_mut)]
    let mut ctx = Egl {
         BindAPI: FnPtr::new(loadfn("eglBindAPI")),
         BindTexImage: FnPtr::new(loadfn("eglBindTexImage")),
         BindWaylandDisplayWL: FnPtr::new(loadfn("eglBindWaylandDisplayWL")),
         ChooseConfig: FnPtr::new(loadfn("eglChooseConfig")),
         ClientSignalSyncEXT: FnPtr::new(loadfn("eglClientSignalSyncEXT")),
         ClientWaitSync: FnPtr::new(loadfn("eglClientWaitSync")),
         ClientWaitSyncKHR: FnPtr::new(loadfn("eglClientWaitSyncKHR")),
         ClientWaitSyncNV: FnPtr::new(loadfn("eglClientWaitSyncNV")),
         CompositorBindTexWindowEXT: FnPtr::new(loadfn("eglCompositorBindTexWindowEXT")),
         CompositorSetContextAttributesEXT: FnPtr::new(loadfn("eglCompositorSetContextAttributesEXT")),
         CompositorSetContextListEXT: FnPtr::new(loadfn("eglCompositorSetContextListEXT")),
         CompositorSetSizeEXT: FnPtr::new(loadfn("eglCompositorSetSizeEXT")),
         CompositorSetWindowAttributesEXT: FnPtr::new(loadfn("eglCompositorSetWindowAttributesEXT")),
         CompositorSetWindowListEXT: FnPtr::new(loadfn("eglCompositorSetWindowListEXT")),
         CompositorSwapPolicyEXT: FnPtr::new(loadfn("eglCompositorSwapPolicyEXT")),
         CopyBuffers: FnPtr::new(loadfn("eglCopyBuffers")),
         CreateContext: FnPtr::new(loadfn("eglCreateContext")),
         CreateDRMImageMESA: FnPtr::new(loadfn("eglCreateDRMImageMESA")),
         CreateFenceSyncNV: FnPtr::new(loadfn("eglCreateFenceSyncNV")),
         CreateImage: FnPtr::new(loadfn("eglCreateImage")),
         CreateImageKHR: FnPtr::new(loadfn("eglCreateImageKHR")),
         CreateNativeClientBufferANDROID: FnPtr::new(loadfn("eglCreateNativeClientBufferANDROID")),
         CreatePbufferFromClientBuffer: FnPtr::new(loadfn("eglCreatePbufferFromClientBuffer")),
         CreatePbufferSurface: FnPtr::new(loadfn("eglCreatePbufferSurface")),
         CreatePixmapSurface: FnPtr::new(loadfn("eglCreatePixmapSurface")),
         CreatePixmapSurfaceHI: FnPtr::new(loadfn("eglCreatePixmapSurfaceHI")),
         CreatePlatformPixmapSurface: FnPtr::new(loadfn("eglCreatePlatformPixmapSurface")),
         CreatePlatformPixmapSurfaceEXT: FnPtr::new(loadfn("eglCreatePlatformPixmapSurfaceEXT")),
         CreatePlatformWindowSurface: FnPtr::new(loadfn("eglCreatePlatformWindowSurface")),
         CreatePlatformWindowSurfaceEXT: FnPtr::new(loadfn("eglCreatePlatformWindowSurfaceEXT")),
         CreateStreamAttribKHR: FnPtr::new(loadfn("eglCreateStreamAttribKHR")),
         CreateStreamFromFileDescriptorKHR: FnPtr::new(loadfn("eglCreateStreamFromFileDescriptorKHR")),
         CreateStreamKHR: FnPtr::new(loadfn("eglCreateStreamKHR")),
         CreateStreamProducerSurfaceKHR: FnPtr::new(loadfn("eglCreateStreamProducerSurfaceKHR")),
         CreateStreamSyncNV: FnPtr::new(loadfn("eglCreateStreamSyncNV")),
         CreateSync: FnPtr::new(loadfn("eglCreateSync")),
         CreateSync64KHR: FnPtr::new(loadfn("eglCreateSync64KHR")),
         CreateSyncKHR: FnPtr::new(loadfn("eglCreateSyncKHR")),
         CreateWaylandBufferFromImageWL: FnPtr::new(loadfn("eglCreateWaylandBufferFromImageWL")),
         CreateWindowSurface: FnPtr::new(loadfn("eglCreateWindowSurface")),
         DebugMessageControlKHR: FnPtr::new(loadfn("eglDebugMessageControlKHR")),
         DestroyContext: FnPtr::new(loadfn("eglDestroyContext")),
         DestroyImage: FnPtr::new(loadfn("eglDestroyImage")),
         DestroyImageKHR: FnPtr::new(loadfn("eglDestroyImageKHR")),
         DestroyStreamKHR: FnPtr::new(loadfn("eglDestroyStreamKHR")),
         DestroySurface: FnPtr::new(loadfn("eglDestroySurface")),
         DestroySync: FnPtr::new(loadfn("eglDestroySync")),
         DestroySyncKHR: FnPtr::new(loadfn("eglDestroySyncKHR")),
         DestroySyncNV: FnPtr::new(loadfn("eglDestroySyncNV")),
         DupNativeFenceFDANDROID: FnPtr::new(loadfn("eglDupNativeFenceFDANDROID")),
         ExportDMABUFImageMESA: FnPtr::new(loadfn("eglExportDMABUFImageMESA")),
         ExportDMABUFImageQueryMESA: FnPtr::new(loadfn("eglExportDMABUFImageQueryMESA")),
         ExportDRMImageMESA: FnPtr::new(loadfn("eglExportDRMImageMESA")),
         FenceNV: FnPtr::new(loadfn("eglFenceNV")),
         GetCompositorTimingANDROID: FnPtr::new(loadfn("eglGetCompositorTimingANDROID")),
         GetCompositorTimingSupportedANDROID: FnPtr::new(loadfn("eglGetCompositorTimingSupportedANDROID")),
         GetConfigAttrib: FnPtr::new(loadfn("eglGetConfigAttrib")),
         GetConfigs: FnPtr::new(loadfn("eglGetConfigs")),
         GetCurrentContext: FnPtr::new(loadfn("eglGetCurrentContext")),
         GetCurrentDisplay: FnPtr::new(loadfn("eglGetCurrentDisplay")),
         GetCurrentSurface: FnPtr::new(loadfn("eglGetCurrentSurface")),
         GetDisplay: FnPtr::new(loadfn("eglGetDisplay")),
         GetDisplayDriverConfig: FnPtr::new(loadfn("eglGetDisplayDriverConfig")),
         GetDisplayDriverName: FnPtr::new(loadfn("eglGetDisplayDriverName")),
         GetError: FnPtr::new(loadfn("eglGetError")),
         GetFrameTimestampSupportedANDROID: FnPtr::new(loadfn("eglGetFrameTimestampSupportedANDROID")),
         GetFrameTimestampsANDROID: FnPtr::new(loadfn("eglGetFrameTimestampsANDROID")),
         GetMscRateANGLE: FnPtr::new(loadfn("eglGetMscRateANGLE")),
         GetNativeClientBufferANDROID: FnPtr::new(loadfn("eglGetNativeClientBufferANDROID")),
         GetNextFrameIdANDROID: FnPtr::new(loadfn("eglGetNextFrameIdANDROID")),
         GetOutputLayersEXT: FnPtr::new(loadfn("eglGetOutputLayersEXT")),
         GetOutputPortsEXT: FnPtr::new(loadfn("eglGetOutputPortsEXT")),
         GetPlatformDisplay: FnPtr::new(loadfn("eglGetPlatformDisplay")),
         GetPlatformDisplayEXT: FnPtr::new(loadfn("eglGetPlatformDisplayEXT")),
         GetProcAddress: FnPtr::new(loadfn("eglGetProcAddress")),
         GetStreamFileDescriptorKHR: FnPtr::new(loadfn("eglGetStreamFileDescriptorKHR")),
         GetSyncAttrib: FnPtr::new(loadfn("eglGetSyncAttrib")),
         GetSyncAttribKHR: FnPtr::new(loadfn("eglGetSyncAttribKHR")),
         GetSyncAttribNV: FnPtr::new(loadfn("eglGetSyncAttribNV")),
         GetSystemTimeFrequencyNV: FnPtr::new(loadfn("eglGetSystemTimeFrequencyNV")),
         GetSystemTimeNV: FnPtr::new(loadfn("eglGetSystemTimeNV")),
         Initialize: FnPtr::new(loadfn("eglInitialize")),
         LabelObjectKHR: FnPtr::new(loadfn("eglLabelObjectKHR")),
         LockSurfaceKHR: FnPtr::new(loadfn("eglLockSurfaceKHR")),
         MakeCurrent: FnPtr::new(loadfn("eglMakeCurrent")),
         OutputLayerAttribEXT: FnPtr::new(loadfn("eglOutputLayerAttribEXT")),
         OutputPortAttribEXT: FnPtr::new(loadfn("eglOutputPortAttribEXT")),
         PostSubBufferNV: FnPtr::new(loadfn("eglPostSubBufferNV")),
         PresentationTimeANDROID: FnPtr::new(loadfn("eglPresentationTimeANDROID")),
         QueryAPI: FnPtr::new(loadfn("eglQueryAPI")),
         QueryContext: FnPtr::new(loadfn("eglQueryContext")),
         QueryDebugKHR: FnPtr::new(loadfn("eglQueryDebugKHR")),
         QueryDeviceAttribEXT: FnPtr::new(loadfn("eglQueryDeviceAttribEXT")),
         QueryDeviceBinaryEXT: FnPtr::new(loadfn("eglQueryDeviceBinaryEXT")),
         QueryDeviceStringEXT: FnPtr::new(loadfn("eglQueryDeviceStringEXT")),
         QueryDevicesEXT: FnPtr::new(loadfn("eglQueryDevicesEXT")),
         QueryDisplayAttribEXT: FnPtr::new(loadfn("eglQueryDisplayAttribEXT")),
         QueryDisplayAttribKHR: FnPtr::new(loadfn("eglQueryDisplayAttribKHR")),
         QueryDisplayAttribNV: FnPtr::new(loadfn("eglQueryDisplayAttribNV")),
         QueryDmaBufFormatsEXT: FnPtr::new(loadfn("eglQueryDmaBufFormatsEXT")),
         QueryDmaBufModifiersEXT: FnPtr::new(loadfn("eglQueryDmaBufModifiersEXT")),
         QueryNativeDisplayNV: FnPtr::new(loadfn("eglQueryNativeDisplayNV")),
         QueryNativePixmapNV: FnPtr::new(loadfn("eglQueryNativePixmapNV")),
         QueryNativeWindowNV: FnPtr::new(loadfn("eglQueryNativeWindowNV")),
         QueryOutputLayerAttribEXT: FnPtr::new(loadfn("eglQueryOutputLayerAttribEXT")),
         QueryOutputLayerStringEXT: FnPtr::new(loadfn("eglQueryOutputLayerStringEXT")),
         QueryOutputPortAttribEXT: FnPtr::new(loadfn("eglQueryOutputPortAttribEXT")),
         QueryOutputPortStringEXT: FnPtr::new(loadfn("eglQueryOutputPortStringEXT")),
         QueryStreamAttribKHR: FnPtr::new(loadfn("eglQueryStreamAttribKHR")),
         QueryStreamConsumerEventNV: FnPtr::new(loadfn("eglQueryStreamConsumerEventNV")),
         QueryStreamKHR: FnPtr::new(loadfn("eglQueryStreamKHR")),
         QueryStreamMetadataNV: FnPtr::new(loadfn("eglQueryStreamMetadataNV")),
         QueryStreamTimeKHR: FnPtr::new(loadfn("eglQueryStreamTimeKHR")),
         QueryStreamu64KHR: FnPtr::new(loadfn("eglQueryStreamu64KHR")),
         QueryString: FnPtr::new(loadfn("eglQueryString")),
         QuerySupportedCompressionRatesEXT: FnPtr::new(loadfn("eglQuerySupportedCompressionRatesEXT")),
         QuerySurface: FnPtr::new(loadfn("eglQuerySurface")),
         QuerySurface64KHR: FnPtr::new(loadfn("eglQuerySurface64KHR")),
         QuerySurfacePointerANGLE: FnPtr::new(loadfn("eglQuerySurfacePointerANGLE")),
         QueryWaylandBufferWL: FnPtr::new(loadfn("eglQueryWaylandBufferWL")),
         ReleaseTexImage: FnPtr::new(loadfn("eglReleaseTexImage")),
         ReleaseThread: FnPtr::new(loadfn("eglReleaseThread")),
         ResetStreamNV: FnPtr::new(loadfn("eglResetStreamNV")),
         SetBlobCacheFuncsANDROID: FnPtr::new(loadfn("eglSetBlobCacheFuncsANDROID")),
         SetDamageRegionKHR: FnPtr::new(loadfn("eglSetDamageRegionKHR")),
         SetStreamAttribKHR: FnPtr::new(loadfn("eglSetStreamAttribKHR")),
         SetStreamMetadataNV: FnPtr::new(loadfn("eglSetStreamMetadataNV")),
         SignalSyncKHR: FnPtr::new(loadfn("eglSignalSyncKHR")),
         SignalSyncNV: FnPtr::new(loadfn("eglSignalSyncNV")),
         StreamAcquireImageNV: FnPtr::new(loadfn("eglStreamAcquireImageNV")),
         StreamAttribKHR: FnPtr::new(loadfn("eglStreamAttribKHR")),
         StreamConsumerAcquireAttribKHR: FnPtr::new(loadfn("eglStreamConsumerAcquireAttribKHR")),
         StreamConsumerAcquireKHR: FnPtr::new(loadfn("eglStreamConsumerAcquireKHR")),
         StreamConsumerGLTextureExternalAttribsNV: FnPtr::new(loadfn("eglStreamConsumerGLTextureExternalAttribsNV")),
         StreamConsumerGLTextureExternalKHR: FnPtr::new(loadfn("eglStreamConsumerGLTextureExternalKHR")),
         StreamConsumerOutputEXT: FnPtr::new(loadfn("eglStreamConsumerOutputEXT")),
         StreamConsumerReleaseAttribKHR: FnPtr::new(loadfn("eglStreamConsumerReleaseAttribKHR")),
         StreamConsumerReleaseKHR: FnPtr::new(loadfn("eglStreamConsumerReleaseKHR")),
         StreamFlushNV: FnPtr::new(loadfn("eglStreamFlushNV")),
         StreamImageConsumerConnectNV: FnPtr::new(loadfn("eglStreamImageConsumerConnectNV")),
         StreamReleaseImageNV: FnPtr::new(loadfn("eglStreamReleaseImageNV")),
         SurfaceAttrib: FnPtr::new(loadfn("eglSurfaceAttrib")),
         SwapBuffers: FnPtr::new(loadfn("eglSwapBuffers")),
         SwapBuffersRegion2NOK: FnPtr::new(loadfn("eglSwapBuffersRegion2NOK")),
         SwapBuffersRegionNOK: FnPtr::new(loadfn("eglSwapBuffersRegionNOK")),
         SwapBuffersWithDamageEXT: FnPtr::new(loadfn("eglSwapBuffersWithDamageEXT")),
         SwapBuffersWithDamageKHR: FnPtr::new(loadfn("eglSwapBuffersWithDamageKHR")),
         SwapInterval: FnPtr::new(loadfn("eglSwapInterval")),
         Terminate: FnPtr::new(loadfn("eglTerminate")),
         UnbindWaylandDisplayWL: FnPtr::new(loadfn("eglUnbindWaylandDisplayWL")),
         UnlockSurfaceKHR: FnPtr::new(loadfn("eglUnlockSurfaceKHR")),
         UnsignalSyncEXT: FnPtr::new(loadfn("eglUnsignalSyncEXT")),
         WaitClient: FnPtr::new(loadfn("eglWaitClient")),
         WaitGL: FnPtr::new(loadfn("eglWaitGL")),
         WaitNative: FnPtr::new(loadfn("eglWaitNative")),
         WaitSync: FnPtr::new(loadfn("eglWaitSync")),
         WaitSyncKHR: FnPtr::new(loadfn("eglWaitSyncKHR")),
    };

     ctx.ClientWaitSync.aliased(&ctx.ClientWaitSyncKHR);
     ctx.ClientWaitSyncKHR.aliased(&ctx.ClientWaitSync);
     ctx.CreateSync.aliased(&ctx.CreateSync64KHR);
     ctx.CreateSync64KHR.aliased(&ctx.CreateSync);
     ctx.DestroyImage.aliased(&ctx.DestroyImageKHR);
     ctx.DestroyImageKHR.aliased(&ctx.DestroyImage);
     ctx.DestroySync.aliased(&ctx.DestroySyncKHR);
     ctx.DestroySyncKHR.aliased(&ctx.DestroySync);
     ctx.QueryDisplayAttribEXT.aliased(&ctx.QueryDisplayAttribKHR);
     ctx.QueryDisplayAttribEXT.aliased(&ctx.QueryDisplayAttribNV);
     ctx.QueryDisplayAttribKHR.aliased(&ctx.QueryDisplayAttribEXT);
     ctx.QueryDisplayAttribKHR.aliased(&ctx.QueryDisplayAttribNV);
     ctx.QueryDisplayAttribNV.aliased(&ctx.QueryDisplayAttribEXT);
     ctx.QueryDisplayAttribNV.aliased(&ctx.QueryDisplayAttribKHR);

     ctx
}

