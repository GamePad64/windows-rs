#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn BuildDisplayTable(lpallocatebuffer: LPALLOCATEBUFFER, lpallocatemore: LPALLOCATEMORE, lpfreebuffer: LPFREEBUFFER, lpmalloc: super::Com::IMalloc, hinstance: super::super::Foundation::HINSTANCE, cpages: u32, lppage: *mut DTPAGE, ulflags: u32, lpptable: *mut IMAPITable, lpptbldata: *mut ITableData) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeIdleRoutine(ftg: *mut ::core::ffi::c_void, lpfnidle: PFNIDLE, lpvidleparam: *mut ::core::ffi::c_void, priidle: i16, csecidle: u32, iroidle: u16, ircidle: u16);
    pub fn CreateIProp(lpinterface: *mut ::windows_sys::core::GUID, lpallocatebuffer: LPALLOCATEBUFFER, lpallocatemore: LPALLOCATEMORE, lpfreebuffer: LPFREEBUFFER, lpvreserved: *mut ::core::ffi::c_void, lpppropdata: *mut IPropData) -> i32;
    pub fn CreateTable(lpinterface: *mut ::windows_sys::core::GUID, lpallocatebuffer: LPALLOCATEBUFFER, lpallocatemore: LPALLOCATEMORE, lpfreebuffer: LPFREEBUFFER, lpvreserved: *mut ::core::ffi::c_void, ultabletype: u32, ulproptagindexcolumn: u32, lpsproptagarraycolumns: *mut SPropTagArray, lpptabledata: *mut ITableData) -> i32;
    pub fn DeinitMapiUtil();
    pub fn DeregisterIdleRoutine(ftg: *mut ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableIdleRoutine(ftg: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL);
    #[cfg(feature = "Win32_Foundation")]
    pub fn FEqualNames(lpname1: *mut MAPINAMEID, lpname2: *mut MAPINAMEID) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn FPropCompareProp(lpspropvalue1: *mut SPropValue, ulrelop: u32, lpspropvalue2: *mut SPropValue) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn FPropContainsProp(lpspropvaluedst: *mut SPropValue, lpspropvaluesrc: *mut SPropValue, ulfuzzylevel: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FPropExists(lpmapiprop: IMAPIProp, ulproptag: u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn FreePadrlist(lpadrlist: *mut ADRLIST);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn FreeProws(lprows: *mut SRowSet);
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtAddFt(ftaddend1: super::super::Foundation::FILETIME, ftaddend2: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtMulDw(ftmultiplier: u32, ftmultiplicand: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtMulDwDw(ftmultiplicand: u32, ftmultiplier: u32) -> super::super::Foundation::FILETIME;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtNegFt(ft: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtSubFt(ftminuend: super::super::Foundation::FILETIME, ftsubtrahend: super::super::Foundation::FILETIME) -> super::super::Foundation::FILETIME;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FtgRegisterIdleRoutine(lpfnidle: PFNIDLE, lpvidleparam: *mut ::core::ffi::c_void, priidle: i16, csecidle: u32, iroidle: u16) -> *mut ::core::ffi::c_void;
    pub fn HrAddColumns(lptbl: IMAPITable, lpproptagcolumnsnew: *mut SPropTagArray, lpallocatebuffer: LPALLOCATEBUFFER, lpfreebuffer: LPFREEBUFFER) -> ::windows_sys::core::HRESULT;
    pub fn HrAddColumnsEx(lptbl: IMAPITable, lpproptagcolumnsnew: *mut SPropTagArray, lpallocatebuffer: LPALLOCATEBUFFER, lpfreebuffer: LPFREEBUFFER, lpfnfiltercolumns: isize) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HrAllocAdviseSink(lpfncallback: LPNOTIFCALLBACK, lpvcontext: *mut ::core::ffi::c_void, lppadvisesink: *mut IMAPIAdviseSink) -> ::windows_sys::core::HRESULT;
    pub fn HrDispatchNotifications(ulflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HrGetOneProp(lpmapiprop: IMAPIProp, ulproptag: u32, lppprop: *mut *mut SPropValue) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn HrIStorageFromStream(lpunkin: ::windows_sys::core::IUnknown, lpinterface: *mut ::windows_sys::core::GUID, ulflags: u32, lppstorageout: *mut super::Com::StructuredStorage::IStorage) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HrQueryAllRows(lptable: IMAPITable, lpproptags: *mut SPropTagArray, lprestriction: *mut SRestriction, lpsortorderset: *mut SSortOrderSet, crowsmax: i32, lpprows: *mut *mut SRowSet) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HrSetOneProp(lpmapiprop: IMAPIProp, lpprop: *mut SPropValue) -> ::windows_sys::core::HRESULT;
    pub fn HrThisThreadAdviseSink(lpadvisesink: IMAPIAdviseSink, lppadvisesink: *mut IMAPIAdviseSink) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn LPropCompareProp(lpspropvaluea: *mut SPropValue, lpspropvalueb: *mut SPropValue) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn LpValFindProp(ulproptag: u32, cvalues: u32, lpproparray: *mut SPropValue) -> *mut SPropValue;
    pub fn MAPIDeinitIdle();
    #[cfg(feature = "Win32_System_Com")]
    pub fn MAPIGetDefaultMalloc() -> ::core::option::Option<super::Com::IMalloc>;
    pub fn MAPIInitIdle(lpvreserved: *mut ::core::ffi::c_void) -> i32;
    #[cfg(feature = "Win32_System_Com")]
    pub fn OpenStreamOnFile(lpallocatebuffer: LPALLOCATEBUFFER, lpfreebuffer: LPFREEBUFFER, ulflags: u32, lpszfilename: *const i8, lpszprefix: *const i8, lppstream: *mut super::Com::IStream) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn PpropFindProp(lpproparray: *mut SPropValue, cvalues: u32, ulproptag: u32) -> *mut SPropValue;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn PropCopyMore(lpspropvaluedest: *mut SPropValue, lpspropvaluesrc: *mut SPropValue, lpfallocmore: LPALLOCATEMORE, lpvobject: *mut ::core::ffi::c_void) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RTFSync(lpmessage: IMessage, ulflags: u32, lpfmessageupdated: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScCopyNotifications(cnotification: i32, lpnotifications: *mut NOTIFICATION, lpvdst: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScCopyProps(cvalues: i32, lpproparray: *mut SPropValue, lpvdst: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScCountNotifications(cnotifications: i32, lpnotifications: *mut NOTIFICATION, lpcb: *mut u32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScCountProps(cvalues: i32, lpproparray: *mut SPropValue, lpcb: *mut u32) -> i32;
    pub fn ScCreateConversationIndex(cbparent: u32, lpbparent: *mut u8, lpcbconvindex: *mut u32, lppbconvindex: *mut *mut u8) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScDupPropset(cvalues: i32, lpproparray: *mut SPropValue, lpallocatebuffer: LPALLOCATEBUFFER, lppproparray: *mut *mut SPropValue) -> i32;
    pub fn ScInitMapiUtil(ulflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScLocalPathFromUNC(lpszunc: super::super::Foundation::PSTR, lpszlocal: super::super::Foundation::PSTR, cchlocal: u32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScRelocNotifications(cnotification: i32, lpnotifications: *mut NOTIFICATION, lpvbaseold: *mut ::core::ffi::c_void, lpvbasenew: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ScRelocProps(cvalues: i32, lpproparray: *mut SPropValue, lpvbaseold: *mut ::core::ffi::c_void, lpvbasenew: *mut ::core::ffi::c_void, lpcb: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScUNCFromLocalPath(lpszlocal: super::super::Foundation::PSTR, lpszunc: super::super::Foundation::PSTR, cchunc: u32) -> i32;
    pub fn SzFindCh(lpsz: *mut i8, ch: u16) -> *mut i8;
    pub fn SzFindLastCh(lpsz: *mut i8, ch: u16) -> *mut i8;
    pub fn SzFindSz(lpsz: *mut i8, lpszkey: *mut i8) -> *mut i8;
    pub fn UFromSz(lpsz: *mut i8) -> u32;
    pub fn UlAddRef(lpunk: *mut ::core::ffi::c_void) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn UlPropSize(lpspropvalue: *mut SPropValue) -> u32;
    pub fn UlRelease(lpunk: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_System_Com")]
    pub fn WrapCompressedRTFStream(lpcompressedrtfstream: super::Com::IStream, ulflags: u32, lpuncompressedrtfstream: *mut super::Com::IStream) -> ::windows_sys::core::HRESULT;
    pub fn WrapStoreEntryID(ulflags: u32, lpszdllname: *const i8, cborigentry: u32, lporigentry: *const ENTRYID, lpcbwrappedentry: *mut u32, lppwrappedentry: *mut *mut ENTRYID) -> ::windows_sys::core::HRESULT;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct ADRENTRY(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct ADRLIST(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct ADRPARM(i32);
pub type CALLERRELEASE = unsafe extern "system" fn(ulcallerdata: u32, lptbldata: ITableData, lpvue: IMAPITable);
#[repr(C)]
pub struct DTBLBUTTON(i32);
#[repr(C)]
pub struct DTBLCHECKBOX(i32);
#[repr(C)]
pub struct DTBLCOMBOBOX(i32);
#[repr(C)]
pub struct DTBLDDLBX(i32);
#[repr(C)]
pub struct DTBLEDIT(i32);
#[repr(C)]
pub struct DTBLGROUPBOX(i32);
#[repr(C)]
pub struct DTBLLABEL(i32);
#[repr(C)]
pub struct DTBLLBX(i32);
#[repr(C)]
pub struct DTBLMVDDLBX(i32);
#[repr(C)]
pub struct DTBLMVLISTBOX(i32);
#[repr(C)]
pub struct DTBLPAGE(i32);
#[repr(C)]
pub struct DTBLRADIOBUTTON(i32);
#[repr(C)]
pub struct DTCTL(i32);
#[repr(C)]
pub struct DTPAGE(i32);
#[repr(C)]
pub struct ENTRYID(i32);
#[repr(C)]
pub struct ERROR_NOTIFICATION(i32);
#[repr(C)]
pub struct EXTENDED_NOTIFICATION(i32);
pub const E_IMAPI_BURN_VERIFICATION_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600697i32 as _);
pub const E_IMAPI_DF2DATA_CLIENT_NAME_IS_NOT_VALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599672i32 as _);
pub const E_IMAPI_DF2DATA_INVALID_MEDIA_STATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599678i32 as _);
pub const E_IMAPI_DF2DATA_MEDIA_IS_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599674i32 as _);
pub const E_IMAPI_DF2DATA_MEDIA_NOT_BLANK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599675i32 as _);
pub const E_IMAPI_DF2DATA_RECORDER_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599673i32 as _);
pub const E_IMAPI_DF2DATA_STREAM_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599677i32 as _);
pub const E_IMAPI_DF2DATA_STREAM_TOO_LARGE_FOR_CURRENT_MEDIA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599676i32 as _);
pub const E_IMAPI_DF2DATA_WRITE_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599680i32 as _);
pub const E_IMAPI_DF2DATA_WRITE_NOT_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599679i32 as _);
pub const E_IMAPI_DF2RAW_CLIENT_NAME_IS_NOT_VALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599164i32 as _);
pub const E_IMAPI_DF2RAW_DATA_BLOCK_TYPE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599154i32 as _);
pub const E_IMAPI_DF2RAW_MEDIA_IS_NOT_BLANK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599162i32 as _);
pub const E_IMAPI_DF2RAW_MEDIA_IS_NOT_PREPARED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599166i32 as _);
pub const E_IMAPI_DF2RAW_MEDIA_IS_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599161i32 as _);
pub const E_IMAPI_DF2RAW_MEDIA_IS_PREPARED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599165i32 as _);
pub const E_IMAPI_DF2RAW_NOT_ENOUGH_SPACE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599159i32 as _);
pub const E_IMAPI_DF2RAW_NO_RECORDER_SPECIFIED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599158i32 as _);
pub const E_IMAPI_DF2RAW_RECORDER_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599152i32 as _);
pub const E_IMAPI_DF2RAW_STREAM_LEADIN_TOO_SHORT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599153i32 as _);
pub const E_IMAPI_DF2RAW_STREAM_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599155i32 as _);
pub const E_IMAPI_DF2RAW_WRITE_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599168i32 as _);
pub const E_IMAPI_DF2RAW_WRITE_NOT_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599167i32 as _);
pub const E_IMAPI_DF2TAO_CLIENT_NAME_IS_NOT_VALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599409i32 as _);
pub const E_IMAPI_DF2TAO_INVALID_ISRC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599413i32 as _);
pub const E_IMAPI_DF2TAO_INVALID_MCN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599412i32 as _);
pub const E_IMAPI_DF2TAO_MEDIA_IS_NOT_BLANK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599418i32 as _);
pub const E_IMAPI_DF2TAO_MEDIA_IS_NOT_PREPARED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599422i32 as _);
pub const E_IMAPI_DF2TAO_MEDIA_IS_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599417i32 as _);
pub const E_IMAPI_DF2TAO_MEDIA_IS_PREPARED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599421i32 as _);
pub const E_IMAPI_DF2TAO_NOT_ENOUGH_SPACE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599415i32 as _);
pub const E_IMAPI_DF2TAO_NO_RECORDER_SPECIFIED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599414i32 as _);
pub const E_IMAPI_DF2TAO_PROPERTY_FOR_BLANK_MEDIA_ONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599420i32 as _);
pub const E_IMAPI_DF2TAO_RECORDER_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599410i32 as _);
pub const E_IMAPI_DF2TAO_STREAM_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599411i32 as _);
pub const E_IMAPI_DF2TAO_TABLE_OF_CONTENTS_EMPTY_DISC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599419i32 as _);
pub const E_IMAPI_DF2TAO_TRACK_LIMIT_REACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599416i32 as _);
pub const E_IMAPI_DF2TAO_WRITE_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599424i32 as _);
pub const E_IMAPI_DF2TAO_WRITE_NOT_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599423i32 as _);
pub const E_IMAPI_ERASE_CLIENT_NAME_IS_NOT_VALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062598389i32 as _);
pub const E_IMAPI_ERASE_DISC_INFORMATION_TOO_SMALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136340222i32 as _);
pub const E_IMAPI_ERASE_DRIVE_FAILED_ERASE_COMMAND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136340219i32 as _);
pub const E_IMAPI_ERASE_DRIVE_FAILED_SPINUP_COMMAND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136340216i32 as _);
pub const E_IMAPI_ERASE_MEDIA_IS_NOT_ERASABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136340220i32 as _);
pub const E_IMAPI_ERASE_MEDIA_IS_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062598391i32 as _);
pub const E_IMAPI_ERASE_MODE_PAGE_2A_TOO_SMALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136340221i32 as _);
pub const E_IMAPI_ERASE_ONLY_ONE_RECORDER_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136340223i32 as _);
pub const E_IMAPI_ERASE_RECORDER_IN_USE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136340224i32 as _);
pub const E_IMAPI_ERASE_RECORDER_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062598390i32 as _);
pub const E_IMAPI_ERASE_TOOK_LONGER_THAN_ONE_HOUR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136340218i32 as _);
pub const E_IMAPI_ERASE_UNEXPECTED_DRIVE_RESPONSE_DURING_ERASE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136340217i32 as _);
pub const E_IMAPI_LOSS_OF_STREAMING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599936i32 as _);
pub const E_IMAPI_RAW_IMAGE_INSUFFICIENT_SPACE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136339963i32 as _);
pub const E_IMAPI_RAW_IMAGE_IS_READ_ONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136339968i32 as _);
pub const E_IMAPI_RAW_IMAGE_NO_TRACKS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136339965i32 as _);
pub const E_IMAPI_RAW_IMAGE_SECTOR_TYPE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136339966i32 as _);
pub const E_IMAPI_RAW_IMAGE_TOO_MANY_TRACKS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136339967i32 as _);
pub const E_IMAPI_RAW_IMAGE_TOO_MANY_TRACK_INDEXES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136339962i32 as _);
pub const E_IMAPI_RAW_IMAGE_TRACKS_ALREADY_ADDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136339964i32 as _);
pub const E_IMAPI_RAW_IMAGE_TRACK_INDEX_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136339961i32 as _);
pub const E_IMAPI_RAW_IMAGE_TRACK_INDEX_OFFSET_ZERO_CANNOT_BE_CLEARED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136339959i32 as _);
pub const E_IMAPI_RAW_IMAGE_TRACK_INDEX_TOO_CLOSE_TO_OTHER_INDEX: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2136339958i32 as _);
pub const E_IMAPI_RECORDER_CLIENT_NAME_IS_NOT_VALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600175i32 as _);
pub const E_IMAPI_RECORDER_COMMAND_TIMEOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600179i32 as _);
pub const E_IMAPI_RECORDER_DVD_STRUCTURE_NOT_PRESENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600178i32 as _);
pub const E_IMAPI_RECORDER_FEATURE_IS_NOT_CURRENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600181i32 as _);
pub const E_IMAPI_RECORDER_GET_CONFIGURATION_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600180i32 as _);
pub const E_IMAPI_RECORDER_INVALID_MODE_PARAMETERS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600184i32 as _);
pub const E_IMAPI_RECORDER_INVALID_RESPONSE_FROM_DEVICE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599937i32 as _);
pub const E_IMAPI_RECORDER_LOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600176i32 as _);
pub const E_IMAPI_RECORDER_MEDIA_BECOMING_READY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600187i32 as _);
pub const E_IMAPI_RECORDER_MEDIA_BUSY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600185i32 as _);
pub const E_IMAPI_RECORDER_MEDIA_FORMAT_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600186i32 as _);
pub const E_IMAPI_RECORDER_MEDIA_INCOMPATIBLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600189i32 as _);
pub const E_IMAPI_RECORDER_MEDIA_NOT_FORMATTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600174i32 as _);
pub const E_IMAPI_RECORDER_MEDIA_NO_MEDIA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600190i32 as _);
pub const E_IMAPI_RECORDER_MEDIA_SPEED_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600177i32 as _);
pub const E_IMAPI_RECORDER_MEDIA_UPSIDE_DOWN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600188i32 as _);
pub const E_IMAPI_RECORDER_MEDIA_WRITE_PROTECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600183i32 as _);
pub const E_IMAPI_RECORDER_NO_SUCH_FEATURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600182i32 as _);
pub const E_IMAPI_RECORDER_NO_SUCH_MODE_PAGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600191i32 as _);
pub const E_IMAPI_RECORDER_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600701i32 as _);
pub const E_IMAPI_REQUEST_CANCELLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062600702i32 as _);
pub const E_IMAPI_UNEXPECTED_RESPONSE_FROM_DEVICE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062599935i32 as _);
pub const FACILITY_IMAPI2: u32 = 170u32;
#[repr(C)]
pub struct FLATENTRY(i32);
#[repr(C)]
pub struct FLATENTRYLIST(i32);
#[repr(C)]
pub struct FLATMTSIDLIST(i32);
#[cfg(feature = "Win32_Foundation")]
pub type FNIDLE = unsafe extern "system" fn(param0: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[repr(transparent)]
pub struct Gender(pub i32);
pub const genderUnspecified: Gender = Gender(0i32);
pub const genderFemale: Gender = Gender(1i32);
pub const genderMale: Gender = Gender(2i32);
#[repr(transparent)]
pub struct IABContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAddrBook(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAttach(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDistList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMAPIAdviseSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMAPIContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMAPIControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMAPIFolder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMAPIProgress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMAPIProp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMAPIStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMAPITable(pub *mut ::core::ffi::c_void);
pub const IMAPI_E_BAD_MULTISESSION_PARAMETER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555294i32 as _);
pub const IMAPI_E_BOOT_EMULATION_IMAGE_SIZE_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555318i32 as _);
pub const IMAPI_E_BOOT_IMAGE_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555320i32 as _);
pub const IMAPI_E_BOOT_OBJECT_CONFLICT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555319i32 as _);
pub const IMAPI_E_DATA_STREAM_CREATE_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555350i32 as _);
pub const IMAPI_E_DATA_STREAM_INCONSISTENCY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555352i32 as _);
pub const IMAPI_E_DATA_STREAM_READ_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555351i32 as _);
pub const IMAPI_E_DATA_TOO_BIG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555342i32 as _);
pub const IMAPI_E_DIRECTORY_READ_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555349i32 as _);
pub const IMAPI_E_DIR_NOT_EMPTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555382i32 as _);
pub const IMAPI_E_DIR_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555366i32 as _);
pub const IMAPI_E_DISC_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555304i32 as _);
pub const IMAPI_E_DUP_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555374i32 as _);
pub const IMAPI_E_EMPTY_DISC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555312i32 as _);
pub const IMAPI_E_FILE_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555367i32 as _);
pub const IMAPI_E_FILE_SYSTEM_CHANGE_NOT_ALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555293i32 as _);
pub const IMAPI_E_FILE_SYSTEM_FEATURE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555308i32 as _);
pub const IMAPI_E_FILE_SYSTEM_NOT_EMPTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555386i32 as _);
pub const IMAPI_E_FILE_SYSTEM_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555310i32 as _);
pub const IMAPI_E_FILE_SYSTEM_READ_CONSISTENCY_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555309i32 as _);
pub const IMAPI_E_FSI_INTERNAL_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555392i32 as _);
pub const IMAPI_E_IMAGEMANAGER_IMAGE_NOT_ALIGNED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555136i32 as _);
pub const IMAPI_E_IMAGEMANAGER_IMAGE_TOO_BIG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555133i32 as _);
pub const IMAPI_E_IMAGEMANAGER_NO_IMAGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555134i32 as _);
pub const IMAPI_E_IMAGEMANAGER_NO_VALID_VD_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555135i32 as _);
pub const IMAPI_E_IMAGE_SIZE_LIMIT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555360i32 as _);
pub const IMAPI_E_IMAGE_TOO_BIG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555359i32 as _);
pub const IMAPI_E_IMPORT_MEDIA_NOT_ALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555303i32 as _);
pub const IMAPI_E_IMPORT_READ_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555305i32 as _);
pub const IMAPI_E_IMPORT_SEEK_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555306i32 as _);
pub const IMAPI_E_IMPORT_TYPE_COLLISION_DIRECTORY_EXISTS_AS_FILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555298i32 as _);
pub const IMAPI_E_IMPORT_TYPE_COLLISION_FILE_EXISTS_AS_DIRECTORY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555307i32 as _);
pub const IMAPI_E_INCOMPATIBLE_MULTISESSION_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555301i32 as _);
pub const IMAPI_E_INCOMPATIBLE_PREVIOUS_SESSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555341i32 as _);
pub const IMAPI_E_INVALID_DATE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555387i32 as _);
pub const IMAPI_E_INVALID_PARAM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555391i32 as _);
pub const IMAPI_E_INVALID_PATH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555376i32 as _);
pub const IMAPI_E_INVALID_VOLUME_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555388i32 as _);
pub const IMAPI_E_INVALID_WORKING_DIRECTORY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555328i32 as _);
pub const IMAPI_E_ISO9660_LEVELS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555343i32 as _);
pub const IMAPI_E_ITEM_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555368i32 as _);
pub const IMAPI_E_MULTISESSION_NOT_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555299i32 as _);
pub const IMAPI_E_NOT_DIR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555383i32 as _);
pub const IMAPI_E_NOT_FILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555384i32 as _);
pub const IMAPI_E_NOT_IN_FILE_SYSTEM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555381i32 as _);
pub const IMAPI_E_NO_COMPATIBLE_MULTISESSION_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555300i32 as _);
pub const IMAPI_E_NO_OUTPUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555389i32 as _);
pub const IMAPI_E_NO_SUPPORTED_FILE_SYSTEM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555311i32 as _);
pub const IMAPI_E_NO_UNIQUE_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555373i32 as _);
pub const IMAPI_E_PROPERTY_NOT_ACCESSIBLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555296i32 as _);
pub const IMAPI_E_READONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555390i32 as _);
pub const IMAPI_E_RESTRICTED_NAME_VIOLATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555375i32 as _);
pub const IMAPI_E_STASHFILE_MOVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555326i32 as _);
pub const IMAPI_E_STASHFILE_OPEN_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555336i32 as _);
pub const IMAPI_E_STASHFILE_READ_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555333i32 as _);
pub const IMAPI_E_STASHFILE_SEEK_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555335i32 as _);
pub const IMAPI_E_STASHFILE_WRITE_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555334i32 as _);
pub const IMAPI_E_TOO_MANY_DIRS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555344i32 as _);
pub const IMAPI_E_UDF_NOT_WRITE_COMPATIBLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555302i32 as _);
pub const IMAPI_E_UDF_REVISION_CHANGE_NOT_ALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555295i32 as _);
pub const IMAPI_E_WORKING_DIRECTORY_SPACE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1062555327i32 as _);
pub const IMAPI_S_IMAGE_FEATURE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(11186527i32 as _);
#[repr(transparent)]
pub struct IMailUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMsgStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProfSect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPropData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProviderAdmin(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITableData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWABExtInit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWABOBJECT_(pub *mut ::core::ffi::c_void);
pub type IWABOBJECT_AddRef_METHOD = unsafe extern "system" fn() -> u32;
pub type IWABOBJECT_AllocateBuffer_METHOD = unsafe extern "system" fn(cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type IWABOBJECT_AllocateMore_METHOD = unsafe extern "system" fn(cbsize: u32, lpobject: *const ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_Backup_METHOD = unsafe extern "system" fn(lpfilename: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_Find_METHOD = unsafe extern "system" fn(lpiab: IAddrBook, hwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT;
pub type IWABOBJECT_FreeBuffer_METHOD = unsafe extern "system" fn(lpbuffer: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type IWABOBJECT_GetLastError_METHOD = unsafe extern "system" fn(hresult: ::windows_sys::core::HRESULT, ulflags: u32, lppmapierror: *mut *mut MAPIERROR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_GetMe_METHOD = unsafe extern "system" fn(lpiab: IAddrBook, ulflags: u32, lpdwaction: *mut u32, lpsbeid: *mut SBinary, hwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_Import_METHOD = unsafe extern "system" fn(lpwip: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_LDAPUrl_METHOD = unsafe extern "system" fn(lpiab: IAddrBook, hwnd: super::super::Foundation::HWND, ulflags: u32, lpszurl: super::super::Foundation::PSTR, lppmailuser: *mut IMailUser) -> ::windows_sys::core::HRESULT;
pub type IWABOBJECT_QueryInterface_METHOD = unsafe extern "system" fn(riid: *const ::windows_sys::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type IWABOBJECT_Release_METHOD = unsafe extern "system" fn() -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_SetMe_METHOD = unsafe extern "system" fn(lpiab: IAddrBook, ulflags: u32, sbeid: SBinary, hwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_VCardCreate_METHOD = unsafe extern "system" fn(lpiab: IAddrBook, ulflags: u32, lpszvcard: super::super::Foundation::PSTR, lpmailuser: IMailUser) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_VCardDisplay_METHOD = unsafe extern "system" fn(lpiab: IAddrBook, hwnd: super::super::Foundation::HWND, lpszfilename: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type IWABOBJECT_VCardRetrieve_METHOD = unsafe extern "system" fn(lpiab: IAddrBook, ulflags: u32, lpszvcard: super::super::Foundation::PSTR, lppmailuser: *mut IMailUser) -> ::windows_sys::core::HRESULT;
#[repr(transparent)]
pub struct IWABObject(pub *mut ::core::ffi::c_void);
pub type LPALLOCATEBUFFER = unsafe extern "system" fn(cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> i32;
pub type LPALLOCATEMORE = unsafe extern "system" fn(cbsize: u32, lpobject: *mut ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> i32;
pub type LPCREATECONVERSATIONINDEX = unsafe extern "system" fn(cbparent: u32, lpbparent: *mut u8, lpcbconvindex: *mut u32, lppbconvindex: *mut *mut u8) -> i32;
pub type LPDISPATCHNOTIFICATIONS = unsafe extern "system" fn(ulflags: u32) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type LPFNABSDI = unsafe extern "system" fn(uluiparam: usize, lpvmsg: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
pub type LPFNBUTTON = unsafe extern "system" fn(uluiparam: usize, lpvcontext: *mut ::core::ffi::c_void, cbentryid: u32, lpselection: *mut ENTRYID, ulflags: u32) -> i32;
pub type LPFNDISMISS = unsafe extern "system" fn(uluiparam: usize, lpvcontext: *mut ::core::ffi::c_void);
pub type LPFREEBUFFER = unsafe extern "system" fn(lpbuffer: *mut ::core::ffi::c_void) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub type LPNOTIFCALLBACK = unsafe extern "system" fn(lpvcontext: *mut ::core::ffi::c_void, cnotification: u32, lpnotifications: *mut NOTIFICATION) -> i32;
#[cfg(feature = "Win32_System_Com")]
pub type LPOPENSTREAMONFILE = unsafe extern "system" fn(lpallocatebuffer: LPALLOCATEBUFFER, lpfreebuffer: LPFREEBUFFER, ulflags: u32, lpszfilename: *const i8, lpszprefix: *const i8, lppstream: *mut super::Com::IStream) -> ::windows_sys::core::HRESULT;
pub type LPWABALLOCATEBUFFER = unsafe extern "system" fn(lpwabobject: IWABObject, cbsize: u32, lppbuffer: *mut *mut ::core::ffi::c_void) -> i32;
pub type LPWABALLOCATEMORE = unsafe extern "system" fn(lpwabobject: IWABObject, cbsize: u32, lpobject: *mut ::core::ffi::c_void, lppbuffer: *mut *mut ::core::ffi::c_void) -> i32;
pub type LPWABFREEBUFFER = unsafe extern "system" fn(lpwabobject: IWABObject, lpbuffer: *mut ::core::ffi::c_void) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWABOPEN = unsafe extern "system" fn(lppadrbook: *mut IAddrBook, lppwabobject: *mut IWABObject, lpwp: *mut WAB_PARAM, reserved2: u32) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type LPWABOPENEX = unsafe extern "system" fn(lppadrbook: *mut IAddrBook, lppwabobject: *mut IWABObject, lpwp: *mut WAB_PARAM, reserved: u32, fnallocatebuffer: LPALLOCATEBUFFER, fnallocatemore: LPALLOCATEMORE, fnfreebuffer: LPFREEBUFFER) -> ::windows_sys::core::HRESULT;
#[repr(C)]
pub struct MAPIERROR(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MAPINAMEID(i32);
#[repr(C)]
pub struct MAPIUID(i32);
pub const MAPI_COMPOUND: u32 = 128u32;
pub const MAPI_DIM: u32 = 1u32;
pub const MAPI_ERROR_VERSION: i32 = 0i32;
pub const MAPI_E_CALL_FAILED: i32 = -2147467259i32;
pub const MAPI_E_INTERFACE_NOT_SUPPORTED: i32 = -2147467262i32;
pub const MAPI_E_INVALID_PARAMETER: i32 = -2147024809i32;
pub const MAPI_E_NOT_ENOUGH_MEMORY: i32 = -2147024882i32;
pub const MAPI_E_NO_ACCESS: i32 = -2147024891i32;
pub const MAPI_NOTRECIP: u32 = 64u32;
pub const MAPI_NOTRESERVED: u32 = 8u32;
pub const MAPI_NOW: u32 = 16u32;
pub const MAPI_ONE_OFF_NO_RICH_INFO: u32 = 1u32;
pub const MAPI_P1: u32 = 268435456u32;
pub const MAPI_SHORTTERM: u32 = 128u32;
pub const MAPI_SUBMITTED: u32 = 2147483648u32;
pub const MAPI_THISSESSION: u32 = 32u32;
pub const MAPI_USE_DEFAULT: u32 = 64u32;
pub const MNID_ID: u32 = 0u32;
pub const MNID_STRING: u32 = 1u32;
#[repr(C)]
pub struct MTSID(i32);
pub const MV_FLAG: u32 = 4096u32;
pub const MV_INSTANCE: u32 = 8192u32;
#[repr(C)]
pub struct NEWMAIL_NOTIFICATION(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct NOTIFICATION(i32);
#[repr(C)]
pub struct NOTIFKEY(i32);
#[repr(C)]
pub struct OBJECT_NOTIFICATION(i32);
#[cfg(feature = "Win32_Foundation")]
pub type PFNIDLE = unsafe extern "system" fn() -> super::super::Foundation::BOOL;
pub const PRIHIGHEST: u32 = 32767u32;
pub const PRILOWEST: i32 = -32768i32;
pub const PRIUSER: u32 = 0u32;
pub const PROP_ID_INVALID: u32 = 65535u32;
pub const PROP_ID_NULL: u32 = 0u32;
pub const PROP_ID_SECURE_MAX: u32 = 26623u32;
pub const PROP_ID_SECURE_MIN: u32 = 26608u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct SAndRestriction(i32);
#[repr(C)]
pub struct SAppTimeArray(i32);
#[repr(C)]
pub struct SBinary(i32);
#[repr(C)]
pub struct SBinaryArray(i32);
#[repr(C)]
pub struct SBitMaskRestriction(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct SCommentRestriction(i32);
#[repr(C)]
pub struct SComparePropsRestriction(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct SContentRestriction(i32);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct SCurrencyArray(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SDateTimeArray(i32);
#[repr(C)]
pub struct SDoubleArray(i32);
pub const SERVICE_UI_ALLOWED: u32 = 16u32;
pub const SERVICE_UI_ALWAYS: u32 = 2u32;
#[repr(C)]
pub struct SExistRestriction(i32);
#[repr(C)]
pub struct SGuidArray(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SLPSTRArray(i32);
#[repr(C)]
pub struct SLargeIntegerArray(i32);
#[repr(C)]
pub struct SLongArray(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct SNotRestriction(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct SOrRestriction(i32);
#[repr(C)]
pub struct SPropProblem(i32);
#[repr(C)]
pub struct SPropProblemArray(i32);
#[repr(C)]
pub struct SPropTagArray(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct SPropValue(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct SPropertyRestriction(i32);
#[repr(C)]
pub struct SRealArray(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct SRestriction(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct SRow(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct SRowSet(i32);
#[repr(C)]
pub struct SShortArray(i32);
#[repr(C)]
pub struct SSizeRestriction(i32);
#[repr(C)]
pub struct SSortOrder(i32);
#[repr(C)]
pub struct SSortOrderSet(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct SSubRestriction(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct STATUS_OBJECT_NOTIFICATION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SWStringArray(i32);
pub const S_IMAPI_BOTHADJUSTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(11141126i32 as _);
pub const S_IMAPI_COMMAND_HAS_SENSE_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(11141632i32 as _);
pub const S_IMAPI_RAW_IMAGE_TRACK_INDEX_ALREADY_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(11143688i32 as _);
pub const S_IMAPI_ROTATIONADJUSTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(11141125i32 as _);
pub const S_IMAPI_SPEEDADJUSTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(11141124i32 as _);
pub const S_IMAPI_WRITE_NOT_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(11141890i32 as _);
pub const TABLE_CHANGED: u32 = 1u32;
pub const TABLE_ERROR: u32 = 2u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct TABLE_NOTIFICATION(i32);
pub const TABLE_RELOAD: u32 = 9u32;
pub const TABLE_RESTRICT_DONE: u32 = 7u32;
pub const TABLE_ROW_ADDED: u32 = 3u32;
pub const TABLE_ROW_DELETED: u32 = 4u32;
pub const TABLE_ROW_MODIFIED: u32 = 5u32;
pub const TABLE_SETCOL_DONE: u32 = 8u32;
pub const TABLE_SORT_DONE: u32 = 6u32;
pub const TAD_ALL_ROWS: u32 = 1u32;
pub const UI_CURRENT_PROVIDER_FIRST: u32 = 4u32;
pub const UI_SERVICE: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WABEXTDISPLAY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WABIMPORTPARAM(i32);
pub const WABOBJECT_LDAPURL_RETURN_MAILUSER: u32 = 1u32;
pub const WABOBJECT_ME_NEW: u32 = 1u32;
pub const WABOBJECT_ME_NOCREATE: u32 = 2u32;
pub const WAB_CONTEXT_ADRLIST: u32 = 2u32;
pub const WAB_DISPLAY_ISNTDS: u32 = 4u32;
pub const WAB_DISPLAY_LDAPURL: u32 = 1u32;
pub const WAB_ENABLE_PROFILES: u32 = 4194304u32;
pub const WAB_IGNORE_PROFILES: u32 = 8388608u32;
pub const WAB_LOCAL_CONTAINERS: u32 = 1048576u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WAB_PARAM(i32);
pub const WAB_PROFILE_CONTENTS: u32 = 2097152u32;
pub const WAB_USE_OE_SENDMAIL: u32 = 1u32;
pub const WAB_VCARD_FILE: u32 = 0u32;
pub const WAB_VCARD_STREAM: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct _PV(i32);
#[repr(C)]
pub struct _WABACTIONITEM(i32);
#[repr(C)]
pub struct _flaglist(i32);