#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISpatialGraphInteropFrameOfReferencePreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialGraphInteropPreviewStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISpatialGraphInteropPreviewStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SpatialGraphInteropFrameOfReferencePreview(pub *mut ::core::ffi::c_void);