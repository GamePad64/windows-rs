#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn AcceptEx(slistensocket: SOCKET, sacceptsocket: SOCKET, lpoutputbuffer: *mut ::core::ffi::c_void, dwreceivedatalength: u32, dwlocaladdresslength: u32, dwremoteaddresslength: u32, lpdwbytesreceived: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    pub fn EnumProtocolsA(lpiprotocols: *const i32, lpprotocolbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32) -> i32;
    pub fn EnumProtocolsW(lpiprotocols: *const i32, lpprotocolbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeAddrInfoEx(paddrinfoex: *const addrinfoexA);
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeAddrInfoExW(paddrinfoex: *const addrinfoexW);
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeAddrInfoW(paddrinfo: *const addrinfoW);
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAcceptExSockaddrs(lpoutputbuffer: *const ::core::ffi::c_void, dwreceivedatalength: u32, dwlocaladdresslength: u32, dwremoteaddresslength: u32, localsockaddr: *mut *mut SOCKADDR, localsockaddrlength: *mut i32, remotesockaddr: *mut *mut SOCKADDR, remotesockaddrlength: *mut i32);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn GetAddrInfoExA(pname: super::super::Foundation::PSTR, pservicename: super::super::Foundation::PSTR, dwnamespace: u32, lpnspid: *const ::windows_sys::core::GUID, hints: *const addrinfoexA, ppresult: *mut *mut addrinfoexA, timeout: *const timeval, lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpcompletionroutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE, lpnamehandle: *mut super::super::Foundation::HANDLE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAddrInfoExCancel(lphandle: *const super::super::Foundation::HANDLE) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn GetAddrInfoExOverlappedResult(lpoverlapped: *const super::super::System::IO::OVERLAPPED) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn GetAddrInfoExW(pname: super::super::Foundation::PWSTR, pservicename: super::super::Foundation::PWSTR, dwnamespace: u32, lpnspid: *const ::windows_sys::core::GUID, hints: *const addrinfoexW, ppresult: *mut *mut addrinfoexW, timeout: *const timeval, lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpcompletionroutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE, lphandle: *mut super::super::Foundation::HANDLE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAddrInfoW(pnodename: super::super::Foundation::PWSTR, pservicename: super::super::Foundation::PWSTR, phints: *const addrinfoW, ppresult: *mut *mut addrinfoW) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAddressByNameA(dwnamespace: u32, lpservicetype: *const ::windows_sys::core::GUID, lpservicename: super::super::Foundation::PSTR, lpiprotocols: *const i32, dwresolution: u32, lpserviceasyncinfo: *const SERVICE_ASYNC_INFO, lpcsaddrbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32, lpaliasbuffer: super::super::Foundation::PSTR, lpdwaliasbufferlength: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAddressByNameW(dwnamespace: u32, lpservicetype: *const ::windows_sys::core::GUID, lpservicename: super::super::Foundation::PWSTR, lpiprotocols: *const i32, dwresolution: u32, lpserviceasyncinfo: *const SERVICE_ASYNC_INFO, lpcsaddrbuffer: *mut ::core::ffi::c_void, lpdwbufferlength: *mut u32, lpaliasbuffer: super::super::Foundation::PWSTR, lpdwaliasbufferlength: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetHostNameW(name: super::super::Foundation::PWSTR, namelen: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNameByTypeA(lpservicetype: *const ::windows_sys::core::GUID, lpservicename: super::super::Foundation::PSTR, dwnamelength: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNameByTypeW(lpservicetype: *const ::windows_sys::core::GUID, lpservicename: super::super::Foundation::PWSTR, dwnamelength: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNameInfoW(psockaddr: *const SOCKADDR, sockaddrlength: i32, pnodebuffer: super::super::Foundation::PWSTR, nodebuffersize: u32, pservicebuffer: super::super::Foundation::PWSTR, servicebuffersize: u32, flags: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetServiceA(dwnamespace: u32, lpguid: *const ::windows_sys::core::GUID, lpservicename: super::super::Foundation::PSTR, dwproperties: u32, lpbuffer: *mut ::core::ffi::c_void, lpdwbuffersize: *mut u32, lpserviceasyncinfo: *const SERVICE_ASYNC_INFO) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetServiceW(dwnamespace: u32, lpguid: *const ::windows_sys::core::GUID, lpservicename: super::super::Foundation::PWSTR, dwproperties: u32, lpbuffer: *mut ::core::ffi::c_void, lpdwbuffersize: *mut u32, lpserviceasyncinfo: *const SERVICE_ASYNC_INFO) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTypeByNameA(lpservicename: super::super::Foundation::PSTR, lpservicetype: *mut ::windows_sys::core::GUID) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTypeByNameW(lpservicename: super::super::Foundation::PWSTR, lpservicetype: *mut ::windows_sys::core::GUID) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InetNtopW(family: i32, paddr: *const ::core::ffi::c_void, pstringbuf: super::super::Foundation::PWSTR, stringbufsize: usize) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InetPtonW(family: i32, pszaddrstring: super::super::Foundation::PWSTR, paddrbuf: *mut ::core::ffi::c_void) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn ProcessSocketNotifications(completionport: super::super::Foundation::HANDLE, registrationcount: u32, registrationinfos: *mut SOCK_NOTIFY_REGISTRATION, timeoutms: u32, completioncount: u32, completionportentries: *mut super::super::System::IO::OVERLAPPED_ENTRY, receivedentrycount: *mut u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WindowsFilteringPlatform"))]
    pub fn RtlEthernetAddressToStringA(addr: *const super::super::NetworkManagement::WindowsFilteringPlatform::DL_EUI48, s: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WindowsFilteringPlatform"))]
    pub fn RtlEthernetAddressToStringW(addr: *const super::super::NetworkManagement::WindowsFilteringPlatform::DL_EUI48, s: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WindowsFilteringPlatform"))]
    pub fn RtlEthernetStringToAddressA(s: super::super::Foundation::PSTR, terminator: *mut super::super::Foundation::PSTR, addr: *mut super::super::NetworkManagement::WindowsFilteringPlatform::DL_EUI48) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WindowsFilteringPlatform"))]
    pub fn RtlEthernetStringToAddressW(s: super::super::Foundation::PWSTR, terminator: *mut super::super::Foundation::PWSTR, addr: *mut super::super::NetworkManagement::WindowsFilteringPlatform::DL_EUI48) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv4AddressToStringA(addr: *const IN_ADDR, s: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv4AddressToStringExA(address: *const IN_ADDR, port: u16, addressstring: super::super::Foundation::PSTR, addressstringlength: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv4AddressToStringExW(address: *const IN_ADDR, port: u16, addressstring: super::super::Foundation::PWSTR, addressstringlength: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv4AddressToStringW(addr: *const IN_ADDR, s: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv4StringToAddressA(s: super::super::Foundation::PSTR, strict: super::super::Foundation::BOOLEAN, terminator: *mut super::super::Foundation::PSTR, addr: *mut IN_ADDR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv4StringToAddressExA(addressstring: super::super::Foundation::PSTR, strict: super::super::Foundation::BOOLEAN, address: *mut IN_ADDR, port: *mut u16) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv4StringToAddressExW(addressstring: super::super::Foundation::PWSTR, strict: super::super::Foundation::BOOLEAN, address: *mut IN_ADDR, port: *mut u16) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv4StringToAddressW(s: super::super::Foundation::PWSTR, strict: super::super::Foundation::BOOLEAN, terminator: *mut super::super::Foundation::PWSTR, addr: *mut IN_ADDR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv6AddressToStringA(addr: *const IN6_ADDR, s: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv6AddressToStringExA(address: *const IN6_ADDR, scopeid: u32, port: u16, addressstring: super::super::Foundation::PSTR, addressstringlength: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv6AddressToStringExW(address: *const IN6_ADDR, scopeid: u32, port: u16, addressstring: super::super::Foundation::PWSTR, addressstringlength: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv6AddressToStringW(addr: *const IN6_ADDR, s: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv6StringToAddressA(s: super::super::Foundation::PSTR, terminator: *mut super::super::Foundation::PSTR, addr: *mut IN6_ADDR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv6StringToAddressExA(addressstring: super::super::Foundation::PSTR, address: *mut IN6_ADDR, scopeid: *mut u32, port: *mut u16) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv6StringToAddressExW(addressstring: super::super::Foundation::PWSTR, address: *mut IN6_ADDR, scopeid: *mut u32, port: *mut u16) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIpv6StringToAddressW(s: super::super::Foundation::PWSTR, terminator: *mut super::super::Foundation::PWSTR, addr: *mut IN6_ADDR) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_IO"))]
    pub fn SetAddrInfoExA(pname: super::super::Foundation::PSTR, pservicename: super::super::Foundation::PSTR, paddresses: *const SOCKET_ADDRESS, dwaddresscount: u32, lpblob: *const super::super::System::Com::BLOB, dwflags: u32, dwnamespace: u32, lpnspid: *const ::windows_sys::core::GUID, timeout: *const timeval, lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpcompletionroutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE, lpnamehandle: *mut super::super::Foundation::HANDLE) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_IO"))]
    pub fn SetAddrInfoExW(pname: super::super::Foundation::PWSTR, pservicename: super::super::Foundation::PWSTR, paddresses: *const SOCKET_ADDRESS, dwaddresscount: u32, lpblob: *const super::super::System::Com::BLOB, dwflags: u32, dwnamespace: u32, lpnspid: *const ::windows_sys::core::GUID, timeout: *const timeval, lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpcompletionroutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE, lpnamehandle: *mut super::super::Foundation::HANDLE) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SetServiceA(dwnamespace: u32, dwoperation: SET_SERVICE_OPERATION, dwflags: u32, lpserviceinfo: *const SERVICE_INFOA, lpserviceasyncinfo: *const SERVICE_ASYNC_INFO, lpdwstatusflags: *mut u32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SetServiceW(dwnamespace: u32, dwoperation: SET_SERVICE_OPERATION, dwflags: u32, lpserviceinfo: *const SERVICE_INFOW, lpserviceasyncinfo: *const SERVICE_ASYNC_INFO, lpdwstatusflags: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSocketMediaStreamingMode(value: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn TransmitFile(hsocket: SOCKET, hfile: super::super::Foundation::HANDLE, nnumberofbytestowrite: u32, nnumberofbytespersend: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lptransmitbuffers: *const TRANSMIT_FILE_BUFFERS, dwreserved: u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WPUCompleteOverlappedRequest(s: SOCKET, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, dwerror: u32, cbtransferred: u32, lperrno: *mut i32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
    pub fn WSAAccept(s: SOCKET, addr: *mut SOCKADDR, addrlen: *mut i32, lpfncondition: LPCONDITIONPROC, dwcallbackdata: usize) -> SOCKET;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAddressToStringA(lpsaaddress: *const SOCKADDR, dwaddresslength: u32, lpprotocolinfo: *const WSAPROTOCOL_INFOA, lpszaddressstring: super::super::Foundation::PSTR, lpdwaddressstringlength: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAddressToStringW(lpsaaddress: *const SOCKADDR, dwaddresslength: u32, lpprotocolinfo: *const WSAPROTOCOL_INFOW, lpszaddressstring: super::super::Foundation::PWSTR, lpdwaddressstringlength: *mut u32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSAAdvertiseProvider(puuidproviderid: *const ::windows_sys::core::GUID, pnspv2routine: *const NSPV2_ROUTINE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAsyncGetHostByAddr(hwnd: super::super::Foundation::HWND, wmsg: u32, addr: super::super::Foundation::PSTR, len: i32, r#type: i32, buf: super::super::Foundation::PSTR, buflen: i32) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAsyncGetHostByName(hwnd: super::super::Foundation::HWND, wmsg: u32, name: super::super::Foundation::PSTR, buf: super::super::Foundation::PSTR, buflen: i32) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAsyncGetProtoByName(hwnd: super::super::Foundation::HWND, wmsg: u32, name: super::super::Foundation::PSTR, buf: super::super::Foundation::PSTR, buflen: i32) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAsyncGetProtoByNumber(hwnd: super::super::Foundation::HWND, wmsg: u32, number: i32, buf: super::super::Foundation::PSTR, buflen: i32) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAsyncGetServByName(hwnd: super::super::Foundation::HWND, wmsg: u32, name: super::super::Foundation::PSTR, proto: super::super::Foundation::PSTR, buf: super::super::Foundation::PSTR, buflen: i32) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAsyncGetServByPort(hwnd: super::super::Foundation::HWND, wmsg: u32, port: i32, proto: super::super::Foundation::PSTR, buf: super::super::Foundation::PSTR, buflen: i32) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAAsyncSelect(s: SOCKET, hwnd: super::super::Foundation::HWND, wmsg: u32, levent: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSACancelAsyncRequest(hasynctaskhandle: super::super::Foundation::HANDLE) -> i32;
    pub fn WSACancelBlockingCall() -> i32;
    pub fn WSACleanup() -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSACloseEvent(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
    pub fn WSAConnect(s: SOCKET, name: *const SOCKADDR, namelen: i32, lpcallerdata: *const WSABUF, lpcalleedata: *mut WSABUF, lpsqos: *const super::super::NetworkManagement::QoS::QOS, lpgqos: *const super::super::NetworkManagement::QoS::QOS) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSAConnectByList(s: SOCKET, socketaddress: *const SOCKET_ADDRESS_LIST, localaddresslength: *mut u32, localaddress: *mut SOCKADDR, remoteaddresslength: *mut u32, remoteaddress: *mut SOCKADDR, timeout: *const timeval, reserved: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSAConnectByNameA(s: SOCKET, nodename: super::super::Foundation::PSTR, servicename: super::super::Foundation::PSTR, localaddresslength: *mut u32, localaddress: *mut SOCKADDR, remoteaddresslength: *mut u32, remoteaddress: *mut SOCKADDR, timeout: *const timeval, reserved: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSAConnectByNameW(s: SOCKET, nodename: super::super::Foundation::PWSTR, servicename: super::super::Foundation::PWSTR, localaddresslength: *mut u32, localaddress: *mut SOCKADDR, remoteaddresslength: *mut u32, remoteaddress: *mut SOCKADDR, timeout: *const timeval, reserved: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSACreateEvent() -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSADeleteSocketPeerTargetName(socket: SOCKET, peeraddr: *const SOCKADDR, peeraddrlen: u32, overlapped: *const super::super::System::IO::OVERLAPPED, completionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSADuplicateSocketA(s: SOCKET, dwprocessid: u32, lpprotocolinfo: *mut WSAPROTOCOL_INFOA) -> i32;
    pub fn WSADuplicateSocketW(s: SOCKET, dwprocessid: u32, lpprotocolinfo: *mut WSAPROTOCOL_INFOW) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAEnumNameSpaceProvidersA(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOA) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSAEnumNameSpaceProvidersExA(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOEXA) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSAEnumNameSpaceProvidersExW(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOEXW) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAEnumNameSpaceProvidersW(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOW) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAEnumNetworkEvents(s: SOCKET, heventobject: super::super::Foundation::HANDLE, lpnetworkevents: *mut WSANETWORKEVENTS) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAEnumProtocolsA(lpiprotocols: *const i32, lpprotocolbuffer: *mut WSAPROTOCOL_INFOA, lpdwbufferlength: *mut u32) -> i32;
    pub fn WSAEnumProtocolsW(lpiprotocols: *const i32, lpprotocolbuffer: *mut WSAPROTOCOL_INFOW, lpdwbufferlength: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAEventSelect(s: SOCKET, heventobject: super::super::Foundation::HANDLE, lnetworkevents: i32) -> i32;
    pub fn WSAGetLastError() -> WSA_ERROR;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSAGetOverlappedResult(s: SOCKET, lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpcbtransfer: *mut u32, fwait: super::super::Foundation::BOOL, lpdwflags: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
    pub fn WSAGetQOSByName(s: SOCKET, lpqosname: *const WSABUF, lpqos: *mut super::super::NetworkManagement::QoS::QOS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAGetServiceClassInfoA(lpproviderid: *const ::windows_sys::core::GUID, lpserviceclassid: *const ::windows_sys::core::GUID, lpdwbufsize: *mut u32, lpserviceclassinfo: *mut WSASERVICECLASSINFOA) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAGetServiceClassInfoW(lpproviderid: *const ::windows_sys::core::GUID, lpserviceclassid: *const ::windows_sys::core::GUID, lpdwbufsize: *mut u32, lpserviceclassinfo: *mut WSASERVICECLASSINFOW) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAGetServiceClassNameByClassIdA(lpserviceclassid: *const ::windows_sys::core::GUID, lpszserviceclassname: super::super::Foundation::PSTR, lpdwbufferlength: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAGetServiceClassNameByClassIdW(lpserviceclassid: *const ::windows_sys::core::GUID, lpszserviceclassname: super::super::Foundation::PWSTR, lpdwbufferlength: *mut u32) -> i32;
    pub fn WSAHtonl(s: SOCKET, hostlong: u32, lpnetlong: *mut u32) -> i32;
    pub fn WSAHtons(s: SOCKET, hostshort: u16, lpnetshort: *mut u16) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAImpersonateSocketPeer(socket: SOCKET, peeraddr: *const SOCKADDR, peeraddrlen: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAInstallServiceClassA(lpserviceclassinfo: *const WSASERVICECLASSINFOA) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAInstallServiceClassW(lpserviceclassinfo: *const WSASERVICECLASSINFOW) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSAIoctl(s: SOCKET, dwiocontrolcode: u32, lpvinbuffer: *const ::core::ffi::c_void, cbinbuffer: u32, lpvoutbuffer: *mut ::core::ffi::c_void, cboutbuffer: u32, lpcbbytesreturned: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAIsBlocking() -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
    pub fn WSAJoinLeaf(s: SOCKET, name: *const SOCKADDR, namelen: i32, lpcallerdata: *const WSABUF, lpcalleedata: *mut WSABUF, lpsqos: *const super::super::NetworkManagement::QoS::QOS, lpgqos: *const super::super::NetworkManagement::QoS::QOS, dwflags: u32) -> SOCKET;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSALookupServiceBeginA(lpqsrestrictions: *const WSAQUERYSETA, dwcontrolflags: u32, lphlookup: *mut super::super::Foundation::HANDLE) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSALookupServiceBeginW(lpqsrestrictions: *const WSAQUERYSETW, dwcontrolflags: u32, lphlookup: *mut super::super::Foundation::HANDLE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSALookupServiceEnd(hlookup: super::super::Foundation::HANDLE) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSALookupServiceNextA(hlookup: super::super::Foundation::HANDLE, dwcontrolflags: u32, lpdwbufferlength: *mut u32, lpqsresults: *mut WSAQUERYSETA) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSALookupServiceNextW(hlookup: super::super::Foundation::HANDLE, dwcontrolflags: u32, lpdwbufferlength: *mut u32, lpqsresults: *mut WSAQUERYSETW) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSANSPIoctl(hlookup: super::super::Foundation::HANDLE, dwcontrolcode: u32, lpvinbuffer: *const ::core::ffi::c_void, cbinbuffer: u32, lpvoutbuffer: *mut ::core::ffi::c_void, cboutbuffer: u32, lpcbbytesreturned: *mut u32, lpcompletion: *const WSACOMPLETION) -> i32;
    pub fn WSANtohl(s: SOCKET, netlong: u32, lphostlong: *mut u32) -> i32;
    pub fn WSANtohs(s: SOCKET, netshort: u16, lphostshort: *mut u16) -> i32;
    pub fn WSAPoll(fdarray: *mut WSAPOLLFD, fds: u32, timeout: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAProviderCompleteAsyncCall(hasynccall: super::super::Foundation::HANDLE, iretcode: i32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSAProviderConfigChange(lpnotificationhandle: *mut super::super::Foundation::HANDLE, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSAQuerySocketSecurity(socket: SOCKET, securityquerytemplate: *const SOCKET_SECURITY_QUERY_TEMPLATE, securityquerytemplatelen: u32, securityqueryinfo: *mut SOCKET_SECURITY_QUERY_INFO, securityqueryinfolen: *mut u32, overlapped: *const super::super::System::IO::OVERLAPPED, completionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSARecv(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytesrecvd: *mut u32, lpflags: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSARecvDisconnect(s: SOCKET, lpinbounddisconnectdata: *const WSABUF) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSARecvEx(s: SOCKET, buf: super::super::Foundation::PSTR, len: i32, flags: *mut i32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSARecvFrom(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytesrecvd: *mut u32, lpflags: *mut u32, lpfrom: *mut SOCKADDR, lpfromlen: *mut i32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32;
    pub fn WSARemoveServiceClass(lpserviceclassid: *const ::windows_sys::core::GUID) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAResetEvent(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    pub fn WSARevertImpersonation() -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSASend(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytessent: *mut u32, dwflags: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSASendDisconnect(s: SOCKET, lpoutbounddisconnectdata: *const WSABUF) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSASendMsg(handle: SOCKET, lpmsg: *const WSAMSG, dwflags: u32, lpnumberofbytessent: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSASendTo(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytessent: *mut u32, dwflags: u32, lpto: *const SOCKADDR, itolen: i32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSASetBlockingHook(lpblockfunc: super::super::Foundation::FARPROC) -> ::core::option::Option<super::super::Foundation::FARPROC>;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSASetEvent(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    pub fn WSASetLastError(ierror: i32);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSASetServiceA(lpqsreginfo: *const WSAQUERYSETA, essoperation: WSAESETSERVICEOP, dwcontrolflags: u32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSASetServiceW(lpqsreginfo: *const WSAQUERYSETW, essoperation: WSAESETSERVICEOP, dwcontrolflags: u32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSASetSocketPeerTargetName(socket: SOCKET, peertargetname: *const SOCKET_PEER_TARGET_NAME, peertargetnamelen: u32, overlapped: *const super::super::System::IO::OVERLAPPED, completionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn WSASetSocketSecurity(socket: SOCKET, securitysettings: *const SOCKET_SECURITY_SETTINGS, securitysettingslen: u32, overlapped: *const super::super::System::IO::OVERLAPPED, completionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSASocketA(af: i32, r#type: i32, protocol: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOA, g: u32, dwflags: u32) -> SOCKET;
    pub fn WSASocketW(af: i32, r#type: i32, protocol: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOW, g: u32, dwflags: u32) -> SOCKET;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAStartup(wversionrequested: u16, lpwsadata: *mut WSAData) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAStringToAddressA(addressstring: super::super::Foundation::PSTR, addressfamily: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOA, lpaddress: *mut SOCKADDR, lpaddresslength: *mut i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAStringToAddressW(addressstring: super::super::Foundation::PWSTR, addressfamily: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOW, lpaddress: *mut SOCKADDR, lpaddresslength: *mut i32) -> i32;
    pub fn WSAUnadvertiseProvider(puuidproviderid: *const ::windows_sys::core::GUID) -> i32;
    pub fn WSAUnhookBlockingHook() -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSAWaitForMultipleEvents(cevents: u32, lphevents: *const super::super::Foundation::HANDLE, fwaitall: super::super::Foundation::BOOL, dwtimeout: u32, falertable: super::super::Foundation::BOOL) -> u32;
    pub fn WSCDeinstallProvider(lpproviderid: *const ::windows_sys::core::GUID, lperrno: *mut i32) -> i32;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn WSCDeinstallProvider32(lpproviderid: *const ::windows_sys::core::GUID, lperrno: *mut i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCEnableNSProvider(lpproviderid: *const ::windows_sys::core::GUID, fenable: super::super::Foundation::BOOL) -> i32;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCEnableNSProvider32(lpproviderid: *const ::windows_sys::core::GUID, fenable: super::super::Foundation::BOOL) -> i32;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCEnumNameSpaceProviders32(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOW) -> i32;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSCEnumNameSpaceProvidersEx32(lpdwbufferlength: *mut u32, lpnspbuffer: *mut WSANAMESPACE_INFOEXW) -> i32;
    pub fn WSCEnumProtocols(lpiprotocols: *const i32, lpprotocolbuffer: *mut WSAPROTOCOL_INFOW, lpdwbufferlength: *mut u32, lperrno: *mut i32) -> i32;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn WSCEnumProtocols32(lpiprotocols: *const i32, lpprotocolbuffer: *mut WSAPROTOCOL_INFOW, lpdwbufferlength: *mut u32, lperrno: *mut i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCGetApplicationCategory(path: super::super::Foundation::PWSTR, pathlength: u32, extra: super::super::Foundation::PWSTR, extralength: u32, ppermittedlspcategories: *mut u32, lperrno: *mut i32) -> i32;
    pub fn WSCGetProviderInfo(lpproviderid: *const ::windows_sys::core::GUID, infotype: WSC_PROVIDER_INFO_TYPE, info: *mut u8, infosize: *mut usize, flags: u32, lperrno: *mut i32) -> i32;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn WSCGetProviderInfo32(lpproviderid: *const ::windows_sys::core::GUID, infotype: WSC_PROVIDER_INFO_TYPE, info: *mut u8, infosize: *mut usize, flags: u32, lperrno: *mut i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCGetProviderPath(lpproviderid: *const ::windows_sys::core::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpproviderdllpathlen: *mut i32, lperrno: *mut i32) -> i32;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCGetProviderPath32(lpproviderid: *const ::windows_sys::core::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpproviderdllpathlen: *mut i32, lperrno: *mut i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCInstallNameSpace(lpszidentifier: super::super::Foundation::PWSTR, lpszpathname: super::super::Foundation::PWSTR, dwnamespace: u32, dwversion: u32, lpproviderid: *const ::windows_sys::core::GUID) -> i32;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCInstallNameSpace32(lpszidentifier: super::super::Foundation::PWSTR, lpszpathname: super::super::Foundation::PWSTR, dwnamespace: u32, dwversion: u32, lpproviderid: *const ::windows_sys::core::GUID) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSCInstallNameSpaceEx(lpszidentifier: super::super::Foundation::PWSTR, lpszpathname: super::super::Foundation::PWSTR, dwnamespace: u32, dwversion: u32, lpproviderid: *const ::windows_sys::core::GUID, lpproviderspecific: *const super::super::System::Com::BLOB) -> i32;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn WSCInstallNameSpaceEx32(lpszidentifier: super::super::Foundation::PWSTR, lpszpathname: super::super::Foundation::PWSTR, dwnamespace: u32, dwversion: u32, lpproviderid: *const ::windows_sys::core::GUID, lpproviderspecific: *const super::super::System::Com::BLOB) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCInstallProvider(lpproviderid: *const ::windows_sys::core::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCInstallProvider64_32(lpproviderid: *const ::windows_sys::core::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCInstallProviderAndChains64_32(lpproviderid: *const ::windows_sys::core::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpszproviderdllpath32: super::super::Foundation::PWSTR, lpszlspname: super::super::Foundation::PWSTR, dwserviceflags: u32, lpprotocolinfolist: *mut WSAPROTOCOL_INFOW, dwnumberofentries: u32, lpdwcatalogentryid: *mut u32, lperrno: *mut i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCSetApplicationCategory(path: super::super::Foundation::PWSTR, pathlength: u32, extra: super::super::Foundation::PWSTR, extralength: u32, permittedlspcategories: u32, pprevpermlspcat: *mut u32, lperrno: *mut i32) -> i32;
    pub fn WSCSetProviderInfo(lpproviderid: *const ::windows_sys::core::GUID, infotype: WSC_PROVIDER_INFO_TYPE, info: *const u8, infosize: usize, flags: u32, lperrno: *mut i32) -> i32;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn WSCSetProviderInfo32(lpproviderid: *const ::windows_sys::core::GUID, infotype: WSC_PROVIDER_INFO_TYPE, info: *const u8, infosize: usize, flags: u32, lperrno: *mut i32) -> i32;
    pub fn WSCUnInstallNameSpace(lpproviderid: *const ::windows_sys::core::GUID) -> i32;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn WSCUnInstallNameSpace32(lpproviderid: *const ::windows_sys::core::GUID) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCUpdateProvider(lpproviderid: *const ::windows_sys::core::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WSCUpdateProvider32(lpproviderid: *const ::windows_sys::core::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32;
    pub fn WSCWriteNameSpaceOrder(lpproviderid: *mut ::windows_sys::core::GUID, dwnumberofentries: u32) -> i32;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn WSCWriteNameSpaceOrder32(lpproviderid: *mut ::windows_sys::core::GUID, dwnumberofentries: u32) -> i32;
    pub fn WSCWriteProviderOrder(lpwdcatalogentryid: *mut u32, dwnumberofentries: u32) -> i32;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn WSCWriteProviderOrder32(lpwdcatalogentryid: *mut u32, dwnumberofentries: u32) -> i32;
    pub fn __WSAFDIsSet(fd: SOCKET, param1: *mut fd_set) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn accept(s: SOCKET, addr: *mut SOCKADDR, addrlen: *mut i32) -> SOCKET;
    #[cfg(feature = "Win32_Foundation")]
    pub fn bind(s: SOCKET, name: *const SOCKADDR, namelen: i32) -> i32;
    pub fn closesocket(s: SOCKET) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn connect(s: SOCKET, name: *const SOCKADDR, namelen: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn freeaddrinfo(paddrinfo: *const ADDRINFOA);
    #[cfg(feature = "Win32_Foundation")]
    pub fn getaddrinfo(pnodename: super::super::Foundation::PSTR, pservicename: super::super::Foundation::PSTR, phints: *const ADDRINFOA, ppresult: *mut *mut ADDRINFOA) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn gethostbyaddr(addr: super::super::Foundation::PSTR, len: i32, r#type: i32) -> *mut hostent;
    #[cfg(feature = "Win32_Foundation")]
    pub fn gethostbyname(name: super::super::Foundation::PSTR) -> *mut hostent;
    #[cfg(feature = "Win32_Foundation")]
    pub fn gethostname(name: super::super::Foundation::PSTR, namelen: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn getnameinfo(psockaddr: *const SOCKADDR, sockaddrlength: i32, pnodebuffer: super::super::Foundation::PSTR, nodebuffersize: u32, pservicebuffer: super::super::Foundation::PSTR, servicebuffersize: u32, flags: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn getpeername(s: SOCKET, name: *mut SOCKADDR, namelen: *mut i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn getprotobyname(name: super::super::Foundation::PSTR) -> *mut protoent;
    #[cfg(feature = "Win32_Foundation")]
    pub fn getprotobynumber(number: i32) -> *mut protoent;
    #[cfg(feature = "Win32_Foundation")]
    pub fn getservbyname(name: super::super::Foundation::PSTR, proto: super::super::Foundation::PSTR) -> *mut servent;
    #[cfg(feature = "Win32_Foundation")]
    pub fn getservbyport(port: i32, proto: super::super::Foundation::PSTR) -> *mut servent;
    #[cfg(feature = "Win32_Foundation")]
    pub fn getsockname(s: SOCKET, name: *mut SOCKADDR, namelen: *mut i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn getsockopt(s: SOCKET, level: i32, optname: i32, optval: super::super::Foundation::PSTR, optlen: *mut i32) -> i32;
    pub fn htonl(hostlong: u32) -> u32;
    pub fn htons(hostshort: u16) -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn inet_addr(cp: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn inet_ntoa(r#in: IN_ADDR) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn inet_ntop(family: i32, paddr: *const ::core::ffi::c_void, pstringbuf: super::super::Foundation::PSTR, stringbufsize: usize) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn inet_pton(family: i32, pszaddrstring: super::super::Foundation::PSTR, paddrbuf: *mut ::core::ffi::c_void) -> i32;
    pub fn ioctlsocket(s: SOCKET, cmd: i32, argp: *mut u32) -> i32;
    pub fn listen(s: SOCKET, backlog: i32) -> i32;
    pub fn ntohl(netlong: u32) -> u32;
    pub fn ntohs(netshort: u16) -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn recv(s: SOCKET, buf: super::super::Foundation::PSTR, len: i32, flags: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn recvfrom(s: SOCKET, buf: super::super::Foundation::PSTR, len: i32, flags: i32, from: *mut SOCKADDR, fromlen: *mut i32) -> i32;
    pub fn select(nfds: i32, readfds: *mut fd_set, writefds: *mut fd_set, exceptfds: *mut fd_set, timeout: *const timeval) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn send(s: SOCKET, buf: super::super::Foundation::PSTR, len: i32, flags: SEND_FLAGS) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn sendto(s: SOCKET, buf: super::super::Foundation::PSTR, len: i32, flags: i32, to: *const SOCKADDR, tolen: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn setsockopt(s: SOCKET, level: i32, optname: i32, optval: super::super::Foundation::PSTR, optlen: i32) -> i32;
    pub fn shutdown(s: SOCKET, how: i32) -> i32;
    pub fn socket(af: i32, r#type: i32, protocol: i32) -> SOCKET;
}
pub const AAL5_MODE_MESSAGE: u32 = 1u32;
pub const AAL5_MODE_STREAMING: u32 = 2u32;
#[repr(C)]
pub struct AAL5_PARAMETERS(i32);
pub const AAL5_SSCS_FRAME_RELAY: u32 = 4u32;
pub const AAL5_SSCS_NULL: u32 = 0u32;
pub const AAL5_SSCS_SSCOP_ASSURED: u32 = 1u32;
pub const AAL5_SSCS_SSCOP_NON_ASSURED: u32 = 2u32;
#[repr(C)]
pub struct AALUSER_PARAMETERS(i32);
#[repr(C)]
pub struct AAL_PARAMETERS_IE(i32);
#[repr(transparent)]
pub struct AAL_TYPE(pub i32);
pub const AALTYPE_5: AAL_TYPE = AAL_TYPE(5i32);
pub const AALTYPE_USER: AAL_TYPE = AAL_TYPE(16i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ADDRINFOA(i32);
pub const ADDRINFOEX_VERSION_2: u32 = 2u32;
pub const ADDRINFOEX_VERSION_3: u32 = 3u32;
pub const ADDRINFOEX_VERSION_4: u32 = 4u32;
pub const ADDRINFOEX_VERSION_5: u32 = 5u32;
pub const ADDRINFOEX_VERSION_6: u32 = 6u32;
#[repr(C)]
pub struct AFPROTOCOLS(i32);
pub const AF_12844: u16 = 25u16;
pub const AF_APPLETALK: u16 = 16u16;
pub const AF_ATM: u16 = 22u16;
pub const AF_BAN: u16 = 21u16;
pub const AF_CCITT: u16 = 10u16;
pub const AF_CHAOS: u16 = 5u16;
pub const AF_CLUSTER: u16 = 24u16;
pub const AF_DATAKIT: u16 = 9u16;
pub const AF_DECnet: u16 = 12u16;
pub const AF_DLI: u16 = 13u16;
pub const AF_ECMA: u16 = 8u16;
pub const AF_FIREFOX: u16 = 19u16;
pub const AF_HYLINK: u16 = 15u16;
pub const AF_HYPERV: u16 = 34u16;
pub const AF_ICLFXBM: u16 = 31u16;
pub const AF_IMPLINK: u16 = 3u16;
pub const AF_IPX: u16 = 6u16;
pub const AF_IRDA: u16 = 26u16;
pub const AF_ISO: u16 = 7u16;
pub const AF_LAT: u16 = 14u16;
pub const AF_LINK: u16 = 33u16;
pub const AF_MAX: u16 = 29u16;
pub const AF_NETBIOS: u16 = 17u16;
pub const AF_NETDES: u16 = 28u16;
pub const AF_NS: u16 = 6u16;
pub const AF_OSI: u16 = 7u16;
pub const AF_PUP: u16 = 4u16;
pub const AF_SNA: u16 = 11u16;
pub const AF_TCNMESSAGE: u16 = 30u16;
pub const AF_TCNPROCESS: u16 = 29u16;
pub const AF_UNIX: u16 = 1u16;
pub const AF_UNKNOWN1: u16 = 20u16;
pub const AF_VOICEVIEW: u16 = 18u16;
pub const AI_ADDRCONFIG: u32 = 1024u32;
pub const AI_ALL: u32 = 256u32;
pub const AI_BYPASS_DNS_CACHE: u32 = 64u32;
pub const AI_CANONNAME: u32 = 2u32;
pub const AI_DISABLE_IDN_ENCODING: u32 = 524288u32;
pub const AI_DNS_ONLY: u32 = 16u32;
pub const AI_DNS_RESPONSE_HOSTFILE: u32 = 2u32;
pub const AI_DNS_RESPONSE_SECURE: u32 = 1u32;
pub const AI_DNS_SERVER_TYPE_DOH: u32 = 2u32;
pub const AI_DNS_SERVER_TYPE_UDP: u32 = 1u32;
pub const AI_DNS_SERVER_UDP_FALLBACK: u32 = 1u32;
pub const AI_EXCLUSIVE_CUSTOM_SERVERS: u32 = 2097152u32;
pub const AI_EXTENDED: u32 = 2147483648u32;
pub const AI_FILESERVER: u32 = 262144u32;
pub const AI_FORCE_CLEAR_TEXT: u32 = 32u32;
pub const AI_FQDN: u32 = 131072u32;
pub const AI_NON_AUTHORITATIVE: u32 = 16384u32;
pub const AI_NUMERICHOST: u32 = 4u32;
pub const AI_NUMERICSERV: u32 = 8u32;
pub const AI_PASSIVE: u32 = 1u32;
pub const AI_REQUIRE_SECURE: u32 = 536870912u32;
pub const AI_RESOLUTION_HANDLE: u32 = 1073741824u32;
pub const AI_RETURN_PREFERRED_NAMES: u32 = 65536u32;
pub const AI_RETURN_RESPONSE_FLAGS: u32 = 268435456u32;
pub const AI_RETURN_TTL: u32 = 128u32;
pub const AI_SECURE: u32 = 32768u32;
pub const AI_SECURE_WITH_FALLBACK: u32 = 1048576u32;
pub const AI_V4MAPPED: u32 = 2048u32;
pub const ASSOCIATE_NAMERES_CONTEXT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1503890279,
    data2: 54526,
    data3: 18145,
    data4: [186, 60, 135, 234, 116, 202, 48, 73],
};
#[repr(C)]
pub struct ASSOCIATE_NAMERES_CONTEXT_INPUT(i32);
pub const ATMPROTO_AAL1: u32 = 1u32;
pub const ATMPROTO_AAL2: u32 = 2u32;
pub const ATMPROTO_AAL34: u32 = 3u32;
pub const ATMPROTO_AAL5: u32 = 5u32;
pub const ATMPROTO_AALUSER: u32 = 0u32;
#[repr(C)]
pub struct ATM_ADDRESS(i32);
pub const ATM_ADDR_SIZE: u32 = 20u32;
pub const ATM_AESA: u32 = 2u32;
#[repr(C)]
pub struct ATM_BHLI(i32);
#[repr(C)]
pub struct ATM_BLLI(i32);
#[repr(C)]
pub struct ATM_BLLI_IE(i32);
#[repr(C)]
pub struct ATM_BROADBAND_BEARER_CAPABILITY_IE(i32);
#[repr(C)]
pub struct ATM_CALLING_PARTY_NUMBER_IE(i32);
#[repr(C)]
pub struct ATM_CAUSE_IE(i32);
#[repr(C)]
pub struct ATM_CONNECTION_ID(i32);
pub const ATM_E164: u32 = 1u32;
pub const ATM_NSAP: u32 = 2u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
#[repr(C)]
pub struct ATM_PVC_PARAMS(i32);
#[repr(C)]
pub struct ATM_QOS_CLASS_IE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ATM_TD(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ATM_TRAFFIC_DESCRIPTOR_IE(i32);
#[repr(C)]
pub struct ATM_TRANSIT_NETWORK_SELECTION_IE(i32);
pub const BASE_PROTOCOL: u32 = 1u32;
pub const BCOB_A: u32 = 1u32;
pub const BCOB_C: u32 = 3u32;
pub const BCOB_X: u32 = 16u32;
pub const BHLI_HighLayerProfile: u32 = 2u32;
pub const BHLI_ISO: u32 = 0u32;
pub const BHLI_UserSpecific: u32 = 1u32;
pub const BHLI_VendorSpecificAppId: u32 = 3u32;
pub const BIGENDIAN: u32 = 0u32;
pub const BITS_PER_BYTE: u32 = 8u32;
pub const BLLI_L2_ELAPB: u32 = 8u32;
pub const BLLI_L2_HDLC_ABM: u32 = 11u32;
pub const BLLI_L2_HDLC_ARM: u32 = 9u32;
pub const BLLI_L2_HDLC_NRM: u32 = 10u32;
pub const BLLI_L2_ISO_1745: u32 = 1u32;
pub const BLLI_L2_ISO_7776: u32 = 17u32;
pub const BLLI_L2_LLC: u32 = 12u32;
pub const BLLI_L2_MODE_EXT: u32 = 128u32;
pub const BLLI_L2_MODE_NORMAL: u32 = 64u32;
pub const BLLI_L2_Q921: u32 = 2u32;
pub const BLLI_L2_Q922: u32 = 14u32;
pub const BLLI_L2_USER_SPECIFIED: u32 = 16u32;
pub const BLLI_L2_X25L: u32 = 6u32;
pub const BLLI_L2_X25M: u32 = 7u32;
pub const BLLI_L2_X75: u32 = 13u32;
pub const BLLI_L3_IPI_IP: u32 = 204u32;
pub const BLLI_L3_IPI_SNAP: u32 = 128u32;
pub const BLLI_L3_ISO_8208: u32 = 7u32;
pub const BLLI_L3_ISO_TR9577: u32 = 11u32;
pub const BLLI_L3_MODE_EXT: u32 = 128u32;
pub const BLLI_L3_MODE_NORMAL: u32 = 64u32;
pub const BLLI_L3_PACKET_1024: u32 = 10u32;
pub const BLLI_L3_PACKET_128: u32 = 7u32;
pub const BLLI_L3_PACKET_16: u32 = 4u32;
pub const BLLI_L3_PACKET_2048: u32 = 11u32;
pub const BLLI_L3_PACKET_256: u32 = 8u32;
pub const BLLI_L3_PACKET_32: u32 = 5u32;
pub const BLLI_L3_PACKET_4096: u32 = 12u32;
pub const BLLI_L3_PACKET_512: u32 = 9u32;
pub const BLLI_L3_PACKET_64: u32 = 6u32;
pub const BLLI_L3_SIO_8473: u32 = 9u32;
pub const BLLI_L3_T70: u32 = 10u32;
pub const BLLI_L3_USER_SPECIFIED: u32 = 16u32;
pub const BLLI_L3_X223: u32 = 8u32;
pub const BLLI_L3_X25: u32 = 6u32;
pub const CAUSE_AAL_PARAMETERS_UNSUPPORTED: u32 = 93u32;
pub const CAUSE_ACCESS_INFORMAION_DISCARDED: u32 = 43u32;
pub const CAUSE_BEARER_CAPABILITY_UNAUTHORIZED: u32 = 57u32;
pub const CAUSE_BEARER_CAPABILITY_UNAVAILABLE: u32 = 58u32;
pub const CAUSE_BEARER_CAPABILITY_UNIMPLEMENTED: u32 = 65u32;
pub const CAUSE_CALL_REJECTED: u32 = 21u32;
pub const CAUSE_CHANNEL_NONEXISTENT: u32 = 82u32;
pub const CAUSE_COND_PERMANENT: u32 = 1u32;
pub const CAUSE_COND_TRANSIENT: u32 = 2u32;
pub const CAUSE_COND_UNKNOWN: u32 = 0u32;
pub const CAUSE_DESTINATION_OUT_OF_ORDER: u32 = 27u32;
pub const CAUSE_INCOMPATIBLE_DESTINATION: u32 = 88u32;
pub const CAUSE_INCORRECT_MESSAGE_LENGTH: u32 = 104u32;
pub const CAUSE_INVALID_CALL_REFERENCE: u32 = 81u32;
pub const CAUSE_INVALID_ENDPOINT_REFERENCE: u32 = 89u32;
pub const CAUSE_INVALID_IE_CONTENTS: u32 = 100u32;
pub const CAUSE_INVALID_NUMBER_FORMAT: u32 = 28u32;
pub const CAUSE_INVALID_STATE_FOR_MESSAGE: u32 = 101u32;
pub const CAUSE_INVALID_TRANSIT_NETWORK_SELECTION: u32 = 91u32;
pub const CAUSE_LOC_BEYOND_INTERWORKING: u32 = 10u32;
pub const CAUSE_LOC_INTERNATIONAL_NETWORK: u32 = 7u32;
pub const CAUSE_LOC_PRIVATE_LOCAL: u32 = 1u32;
pub const CAUSE_LOC_PRIVATE_REMOTE: u32 = 5u32;
pub const CAUSE_LOC_PUBLIC_LOCAL: u32 = 2u32;
pub const CAUSE_LOC_PUBLIC_REMOTE: u32 = 4u32;
pub const CAUSE_LOC_TRANSIT_NETWORK: u32 = 3u32;
pub const CAUSE_LOC_USER: u32 = 0u32;
pub const CAUSE_MANDATORY_IE_MISSING: u32 = 96u32;
pub const CAUSE_NA_ABNORMAL: u32 = 4u32;
pub const CAUSE_NA_NORMAL: u32 = 0u32;
pub const CAUSE_NETWORK_OUT_OF_ORDER: u32 = 38u32;
pub const CAUSE_NORMAL_CALL_CLEARING: u32 = 16u32;
pub const CAUSE_NORMAL_UNSPECIFIED: u32 = 31u32;
pub const CAUSE_NO_ROUTE_TO_DESTINATION: u32 = 3u32;
pub const CAUSE_NO_ROUTE_TO_TRANSIT_NETWORK: u32 = 2u32;
pub const CAUSE_NO_USER_RESPONDING: u32 = 18u32;
pub const CAUSE_NO_VPI_VCI_AVAILABLE: u32 = 45u32;
pub const CAUSE_NUMBER_CHANGED: u32 = 22u32;
pub const CAUSE_OPTION_UNAVAILABLE: u32 = 63u32;
pub const CAUSE_PROTOCOL_ERROR: u32 = 111u32;
pub const CAUSE_PU_PROVIDER: u32 = 0u32;
pub const CAUSE_PU_USER: u32 = 8u32;
pub const CAUSE_QOS_UNAVAILABLE: u32 = 49u32;
pub const CAUSE_REASON_IE_INSUFFICIENT: u32 = 8u32;
pub const CAUSE_REASON_IE_MISSING: u32 = 4u32;
pub const CAUSE_REASON_USER: u32 = 0u32;
pub const CAUSE_RECOVERY_ON_TIMEOUT: u32 = 102u32;
pub const CAUSE_RESOURCE_UNAVAILABLE: u32 = 47u32;
pub const CAUSE_STATUS_ENQUIRY_RESPONSE: u32 = 30u32;
pub const CAUSE_TEMPORARY_FAILURE: u32 = 41u32;
pub const CAUSE_TOO_MANY_PENDING_ADD_PARTY: u32 = 92u32;
pub const CAUSE_UNALLOCATED_NUMBER: u32 = 1u32;
pub const CAUSE_UNIMPLEMENTED_IE: u32 = 99u32;
pub const CAUSE_UNIMPLEMENTED_MESSAGE_TYPE: u32 = 97u32;
pub const CAUSE_UNSUPPORTED_TRAFFIC_PARAMETERS: u32 = 73u32;
pub const CAUSE_USER_BUSY: u32 = 17u32;
pub const CAUSE_USER_CELL_RATE_UNAVAILABLE: u32 = 51u32;
pub const CAUSE_USER_REJECTS_CLIR: u32 = 23u32;
pub const CAUSE_VPI_VCI_UNACCEPTABLE: u32 = 10u32;
pub const CAUSE_VPI_VCI_UNAVAILABLE: u32 = 35u32;
pub const CF_ACCEPT: u32 = 0u32;
pub const CF_DEFER: u32 = 2u32;
pub const CF_REJECT: u32 = 1u32;
pub const CLIP_NOT: u32 = 0u32;
pub const CLIP_SUS: u32 = 32u32;
#[repr(transparent)]
pub struct CONTROL_CHANNEL_TRIGGER_STATUS(pub i32);
pub const CONTROL_CHANNEL_TRIGGER_STATUS_INVALID: CONTROL_CHANNEL_TRIGGER_STATUS = CONTROL_CHANNEL_TRIGGER_STATUS(0i32);
pub const CONTROL_CHANNEL_TRIGGER_STATUS_SOFTWARE_SLOT_ALLOCATED: CONTROL_CHANNEL_TRIGGER_STATUS = CONTROL_CHANNEL_TRIGGER_STATUS(1i32);
pub const CONTROL_CHANNEL_TRIGGER_STATUS_HARDWARE_SLOT_ALLOCATED: CONTROL_CHANNEL_TRIGGER_STATUS = CONTROL_CHANNEL_TRIGGER_STATUS(2i32);
pub const CONTROL_CHANNEL_TRIGGER_STATUS_POLICY_ERROR: CONTROL_CHANNEL_TRIGGER_STATUS = CONTROL_CHANNEL_TRIGGER_STATUS(3i32);
pub const CONTROL_CHANNEL_TRIGGER_STATUS_SYSTEM_ERROR: CONTROL_CHANNEL_TRIGGER_STATUS = CONTROL_CHANNEL_TRIGGER_STATUS(4i32);
pub const CONTROL_CHANNEL_TRIGGER_STATUS_TRANSPORT_DISCONNECTED: CONTROL_CHANNEL_TRIGGER_STATUS = CONTROL_CHANNEL_TRIGGER_STATUS(5i32);
pub const CONTROL_CHANNEL_TRIGGER_STATUS_SERVICE_UNAVAILABLE: CONTROL_CHANNEL_TRIGGER_STATUS = CONTROL_CHANNEL_TRIGGER_STATUS(6i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CSADDR_INFO(i32);
pub const DE_REUSE_SOCKET: u32 = 2u32;
pub const FD_ACCEPT: u32 = 8u32;
pub const FD_ACCEPT_BIT: u32 = 3u32;
pub const FD_ADDRESS_LIST_CHANGE_BIT: u32 = 9u32;
pub const FD_CLOSE: u32 = 32u32;
pub const FD_CLOSE_BIT: u32 = 5u32;
pub const FD_CONNECT: u32 = 16u32;
pub const FD_CONNECT_BIT: u32 = 4u32;
pub const FD_GROUP_QOS_BIT: u32 = 7u32;
pub const FD_MAX_EVENTS: u32 = 10u32;
pub const FD_OOB: u32 = 4u32;
pub const FD_OOB_BIT: u32 = 2u32;
pub const FD_QOS_BIT: u32 = 6u32;
pub const FD_READ: u32 = 1u32;
pub const FD_READ_BIT: u32 = 0u32;
pub const FD_ROUTING_INTERFACE_CHANGE_BIT: u32 = 8u32;
pub const FD_SETSIZE: u32 = 64u32;
pub const FD_WRITE: u32 = 2u32;
pub const FD_WRITE_BIT: u32 = 1u32;
pub const FIOASYNC: i32 = -2147195267i32;
pub const FIONBIO: i32 = -2147195266i32;
pub const FIONREAD: i32 = 1074030207i32;
pub const FROM_PROTOCOL_INFO: i32 = -1i32;
pub const GAI_STRERROR_BUFFER_SIZE: u32 = 1024u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GROUP_FILTER(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GROUP_REQ(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GROUP_SOURCE_REQ(i32);
#[repr(C)]
pub struct HWSAEVENT(i32);
pub const IAS_ATTRIB_INT: u32 = 1u32;
pub const IAS_ATTRIB_NO_ATTRIB: u32 = 0u32;
pub const IAS_ATTRIB_NO_CLASS: u32 = 16u32;
pub const IAS_ATTRIB_OCTETSEQ: u32 = 2u32;
pub const IAS_ATTRIB_STR: u32 = 3u32;
pub const IAS_MAX_ATTRIBNAME: u32 = 256u32;
pub const IAS_MAX_CLASSNAME: u32 = 64u32;
pub const IAS_MAX_OCTET_STRING: u32 = 1024u32;
pub const IAS_MAX_USER_STRING: u32 = 256u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ICMP_ERROR_INFO(i32);
pub const IFF_BROADCAST: u32 = 2u32;
pub const IFF_LOOPBACK: u32 = 4u32;
pub const IFF_MULTICAST: u32 = 16u32;
pub const IFF_POINTTOPOINT: u32 = 8u32;
pub const IFF_UP: u32 = 1u32;
pub const IMPLINK_HIGHEXPER: u32 = 158u32;
pub const IMPLINK_IP: u32 = 155u32;
pub const IMPLINK_LOWEXPER: u32 = 156u32;
pub const IN4ADDR_LINKLOCALPREFIX_LENGTH: u32 = 16u32;
pub const IN4ADDR_LOOPBACK: u32 = 16777343u32;
pub const IN4ADDR_LOOPBACKPREFIX_LENGTH: u32 = 8u32;
pub const IN4ADDR_MULTICASTPREFIX_LENGTH: u32 = 4u32;
pub const IN6ADDR_6TO4PREFIX_LENGTH: u32 = 16u32;
pub const IN6ADDR_LINKLOCALPREFIX_LENGTH: u32 = 64u32;
pub const IN6ADDR_MULTICASTPREFIX_LENGTH: u32 = 8u32;
pub const IN6ADDR_SOLICITEDNODEMULTICASTPREFIX_LENGTH: u32 = 104u32;
pub const IN6ADDR_TEREDOPREFIX_LENGTH: u32 = 32u32;
pub const IN6ADDR_V4MAPPEDPREFIX_LENGTH: u32 = 96u32;
#[repr(C)]
pub struct IN6_ADDR(i32);
#[repr(C)]
pub struct IN6_PKTINFO(i32);
pub const INADDR_LOOPBACK: u32 = 2130706433u32;
pub const INADDR_NONE: u32 = 4294967295u32;
pub const INCL_WINSOCK_API_PROTOTYPES: u32 = 1u32;
pub const INCL_WINSOCK_API_TYPEDEFS: u32 = 0u32;
pub const INET6_ADDRSTRLEN: u32 = 65u32;
pub const INET_ADDRSTRLEN: u32 = 22u32;
#[repr(C)]
pub struct INET_PORT_RANGE(i32);
#[repr(C)]
pub struct INET_PORT_RESERVATION_INFORMATION(i32);
#[repr(C)]
pub struct INET_PORT_RESERVATION_INSTANCE(i32);
#[repr(C)]
pub struct INET_PORT_RESERVATION_TOKEN(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct INTERFACE_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct INTERFACE_INFO_EX(i32);
pub const INVALID_SOCKET: SOCKET = SOCKET(4294967295u32 as _);
#[repr(C)]
pub struct IN_ADDR(i32);
pub const IN_CLASSA_HOST: u32 = 16777215u32;
pub const IN_CLASSA_MAX: u32 = 128u32;
pub const IN_CLASSA_NET: u32 = 4278190080u32;
pub const IN_CLASSA_NSHIFT: u32 = 24u32;
pub const IN_CLASSB_HOST: u32 = 65535u32;
pub const IN_CLASSB_MAX: u32 = 65536u32;
pub const IN_CLASSB_NET: u32 = 4294901760u32;
pub const IN_CLASSB_NSHIFT: u32 = 16u32;
pub const IN_CLASSC_HOST: u32 = 255u32;
pub const IN_CLASSC_NET: u32 = 4294967040u32;
pub const IN_CLASSC_NSHIFT: u32 = 8u32;
pub const IN_CLASSD_HOST: u32 = 268435455u32;
pub const IN_CLASSD_NET: u32 = 4026531840u32;
pub const IN_CLASSD_NSHIFT: u32 = 28u32;
#[repr(C)]
pub struct IN_PKTINFO(i32);
#[repr(C)]
pub struct IN_PKTINFO_EX(i32);
#[repr(C)]
pub struct IN_RECVERR(i32);
pub const IOCPARM_MASK: u32 = 127u32;
pub const IOC_IN: u32 = 2147483648u32;
pub const IOC_INOUT: u32 = 3221225472u32;
pub const IOC_OUT: u32 = 1073741824u32;
pub const IOC_PROTOCOL: u32 = 268435456u32;
pub const IOC_UNIX: u32 = 0u32;
pub const IOC_VENDOR: u32 = 402653184u32;
pub const IOC_VOID: u32 = 536870912u32;
pub const IOC_WS2: u32 = 134217728u32;
pub const IP6T_SO_ORIGINAL_DST: u32 = 12303u32;
pub const IPPORT_BIFFUDP: u32 = 512u32;
pub const IPPORT_CHARGEN: u32 = 19u32;
pub const IPPORT_CMDSERVER: u32 = 514u32;
pub const IPPORT_DAYTIME: u32 = 13u32;
pub const IPPORT_DISCARD: u32 = 9u32;
pub const IPPORT_DYNAMIC_MAX: u32 = 65535u32;
pub const IPPORT_DYNAMIC_MIN: u32 = 49152u32;
pub const IPPORT_ECHO: u32 = 7u32;
pub const IPPORT_EFSSERVER: u32 = 520u32;
pub const IPPORT_EPMAP: u32 = 135u32;
pub const IPPORT_EXECSERVER: u32 = 512u32;
pub const IPPORT_FINGER: u32 = 79u32;
pub const IPPORT_FTP: u32 = 21u32;
pub const IPPORT_FTP_DATA: u32 = 20u32;
pub const IPPORT_HTTPS: u32 = 443u32;
pub const IPPORT_IMAP: u32 = 143u32;
pub const IPPORT_IMAP3: u32 = 220u32;
pub const IPPORT_LDAP: u32 = 389u32;
pub const IPPORT_LOGINSERVER: u32 = 513u32;
pub const IPPORT_MICROSOFT_DS: u32 = 445u32;
pub const IPPORT_MSP: u32 = 18u32;
pub const IPPORT_MTP: u32 = 57u32;
pub const IPPORT_NAMESERVER: u32 = 42u32;
pub const IPPORT_NETBIOS_DGM: u32 = 138u32;
pub const IPPORT_NETBIOS_NS: u32 = 137u32;
pub const IPPORT_NETBIOS_SSN: u32 = 139u32;
pub const IPPORT_NETSTAT: u32 = 15u32;
pub const IPPORT_NTP: u32 = 123u32;
pub const IPPORT_POP3: u32 = 110u32;
pub const IPPORT_QOTD: u32 = 17u32;
pub const IPPORT_REGISTERED_MAX: u32 = 49151u32;
pub const IPPORT_REGISTERED_MIN: u32 = 1024u32;
pub const IPPORT_RESERVED: u32 = 1024u32;
pub const IPPORT_RJE: u32 = 77u32;
pub const IPPORT_ROUTESERVER: u32 = 520u32;
pub const IPPORT_SMTP: u32 = 25u32;
pub const IPPORT_SNMP: u32 = 161u32;
pub const IPPORT_SNMP_TRAP: u32 = 162u32;
pub const IPPORT_SUPDUP: u32 = 95u32;
pub const IPPORT_SYSTAT: u32 = 11u32;
pub const IPPORT_TCPMUX: u32 = 1u32;
pub const IPPORT_TELNET: u32 = 23u32;
pub const IPPORT_TFTP: u32 = 69u32;
pub const IPPORT_TIMESERVER: u32 = 37u32;
pub const IPPORT_TTYLINK: u32 = 87u32;
pub const IPPORT_WHOIS: u32 = 43u32;
pub const IPPORT_WHOSERVER: u32 = 513u32;
#[repr(transparent)]
pub struct IPPROTO(pub i32);
pub const IPPROTO_HOPOPTS: IPPROTO = IPPROTO(0i32);
pub const IPPROTO_ICMP: IPPROTO = IPPROTO(1i32);
pub const IPPROTO_IGMP: IPPROTO = IPPROTO(2i32);
pub const IPPROTO_GGP: IPPROTO = IPPROTO(3i32);
pub const IPPROTO_IPV4: IPPROTO = IPPROTO(4i32);
pub const IPPROTO_ST: IPPROTO = IPPROTO(5i32);
pub const IPPROTO_TCP: IPPROTO = IPPROTO(6i32);
pub const IPPROTO_CBT: IPPROTO = IPPROTO(7i32);
pub const IPPROTO_EGP: IPPROTO = IPPROTO(8i32);
pub const IPPROTO_IGP: IPPROTO = IPPROTO(9i32);
pub const IPPROTO_PUP: IPPROTO = IPPROTO(12i32);
pub const IPPROTO_UDP: IPPROTO = IPPROTO(17i32);
pub const IPPROTO_IDP: IPPROTO = IPPROTO(22i32);
pub const IPPROTO_RDP: IPPROTO = IPPROTO(27i32);
pub const IPPROTO_IPV6: IPPROTO = IPPROTO(41i32);
pub const IPPROTO_ROUTING: IPPROTO = IPPROTO(43i32);
pub const IPPROTO_FRAGMENT: IPPROTO = IPPROTO(44i32);
pub const IPPROTO_ESP: IPPROTO = IPPROTO(50i32);
pub const IPPROTO_AH: IPPROTO = IPPROTO(51i32);
pub const IPPROTO_ICMPV6: IPPROTO = IPPROTO(58i32);
pub const IPPROTO_NONE: IPPROTO = IPPROTO(59i32);
pub const IPPROTO_DSTOPTS: IPPROTO = IPPROTO(60i32);
pub const IPPROTO_ND: IPPROTO = IPPROTO(77i32);
pub const IPPROTO_ICLFXBM: IPPROTO = IPPROTO(78i32);
pub const IPPROTO_PIM: IPPROTO = IPPROTO(103i32);
pub const IPPROTO_PGM: IPPROTO = IPPROTO(113i32);
pub const IPPROTO_L2TP: IPPROTO = IPPROTO(115i32);
pub const IPPROTO_SCTP: IPPROTO = IPPROTO(132i32);
pub const IPPROTO_RAW: IPPROTO = IPPROTO(255i32);
pub const IPPROTO_MAX: IPPROTO = IPPROTO(256i32);
pub const IPPROTO_RESERVED_RAW: IPPROTO = IPPROTO(257i32);
pub const IPPROTO_RESERVED_IPSEC: IPPROTO = IPPROTO(258i32);
pub const IPPROTO_RESERVED_IPSECOFFLOAD: IPPROTO = IPPROTO(259i32);
pub const IPPROTO_RESERVED_WNV: IPPROTO = IPPROTO(260i32);
pub const IPPROTO_RESERVED_MAX: IPPROTO = IPPROTO(261i32);
pub const IPPROTO_IP: u32 = 0u32;
pub const IPPROTO_RM: u32 = 113u32;
pub const IPV6_ADD_IFLIST: u32 = 29u32;
pub const IPV6_ADD_MEMBERSHIP: u32 = 12u32;
pub const IPV6_CHECKSUM: u32 = 26u32;
pub const IPV6_DEL_IFLIST: u32 = 30u32;
pub const IPV6_DONTFRAG: u32 = 14u32;
pub const IPV6_DROP_MEMBERSHIP: u32 = 13u32;
pub const IPV6_ECN: u32 = 50u32;
pub const IPV6_GET_IFLIST: u32 = 33u32;
pub const IPV6_HDRINCL: u32 = 2u32;
pub const IPV6_HOPLIMIT: u32 = 21u32;
pub const IPV6_HOPOPTS: u32 = 1u32;
pub const IPV6_IFLIST: u32 = 28u32;
pub const IPV6_JOIN_GROUP: u32 = 12u32;
pub const IPV6_LEAVE_GROUP: u32 = 13u32;
#[repr(C)]
pub struct IPV6_MREQ(i32);
pub const IPV6_MTU: u32 = 72u32;
pub const IPV6_MTU_DISCOVER: u32 = 71u32;
pub const IPV6_MULTICAST_HOPS: u32 = 10u32;
pub const IPV6_MULTICAST_IF: u32 = 9u32;
pub const IPV6_MULTICAST_LOOP: u32 = 11u32;
pub const IPV6_NRT_INTERFACE: u32 = 74u32;
pub const IPV6_PKTINFO: u32 = 19u32;
pub const IPV6_PKTINFO_EX: u32 = 51u32;
pub const IPV6_PROTECTION_LEVEL: u32 = 23u32;
pub const IPV6_RECVDSTADDR: u32 = 25u32;
pub const IPV6_RECVECN: u32 = 50u32;
pub const IPV6_RECVERR: u32 = 75u32;
pub const IPV6_RECVIF: u32 = 24u32;
pub const IPV6_RECVRTHDR: u32 = 38u32;
pub const IPV6_RECVTCLASS: u32 = 40u32;
pub const IPV6_RTHDR: u32 = 32u32;
pub const IPV6_TCLASS: u32 = 39u32;
pub const IPV6_UNICAST_HOPS: u32 = 4u32;
pub const IPV6_UNICAST_IF: u32 = 31u32;
pub const IPV6_USER_MTU: u32 = 76u32;
pub const IPV6_V6ONLY: u32 = 27u32;
pub const IPV6_WFP_REDIRECT_CONTEXT: u32 = 70u32;
pub const IPV6_WFP_REDIRECT_RECORDS: u32 = 60u32;
pub const IPX_ADDRESS: u32 = 16391u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IPX_ADDRESS_DATA(i32);
pub const IPX_ADDRESS_NOTIFY: u32 = 16396u32;
pub const IPX_DSTYPE: u32 = 16386u32;
pub const IPX_EXTENDED_ADDRESS: u32 = 16388u32;
pub const IPX_FILTERPTYPE: u32 = 16385u32;
pub const IPX_GETNETINFO: u32 = 16392u32;
pub const IPX_GETNETINFO_NORIP: u32 = 16393u32;
pub const IPX_IMMEDIATESPXACK: u32 = 16400u32;
pub const IPX_MAXSIZE: u32 = 16390u32;
pub const IPX_MAX_ADAPTER_NUM: u32 = 16397u32;
#[repr(C)]
pub struct IPX_NETNUM_DATA(i32);
pub const IPX_PTYPE: u32 = 16384u32;
pub const IPX_RECEIVE_BROADCAST: u32 = 16399u32;
pub const IPX_RECVHDR: u32 = 16389u32;
pub const IPX_RERIPNETNUMBER: u32 = 16398u32;
#[repr(C)]
pub struct IPX_SPXCONNSTATUS_DATA(i32);
pub const IPX_SPXGETCONNECTIONSTATUS: u32 = 16395u32;
pub const IPX_STOPFILTERPTYPE: u32 = 16387u32;
pub const IP_ADD_IFLIST: u32 = 29u32;
pub const IP_ADD_MEMBERSHIP: u32 = 12u32;
pub const IP_ADD_SOURCE_MEMBERSHIP: u32 = 15u32;
pub const IP_BLOCK_SOURCE: u32 = 17u32;
pub const IP_DEFAULT_MULTICAST_LOOP: u32 = 1u32;
pub const IP_DEFAULT_MULTICAST_TTL: u32 = 1u32;
pub const IP_DEL_IFLIST: u32 = 30u32;
pub const IP_DONTFRAGMENT: u32 = 14u32;
pub const IP_DROP_MEMBERSHIP: u32 = 13u32;
pub const IP_DROP_SOURCE_MEMBERSHIP: u32 = 16u32;
pub const IP_ECN: u32 = 50u32;
pub const IP_GET_IFLIST: u32 = 33u32;
pub const IP_HDRINCL: u32 = 2u32;
pub const IP_HOPLIMIT: u32 = 21u32;
pub const IP_IFLIST: u32 = 28u32;
pub const IP_MAX_MEMBERSHIPS: u32 = 20u32;
#[repr(C)]
pub struct IP_MREQ(i32);
#[repr(C)]
pub struct IP_MREQ_SOURCE(i32);
#[repr(C)]
pub struct IP_MSFILTER(i32);
pub const IP_MTU: u32 = 73u32;
pub const IP_MTU_DISCOVER: u32 = 71u32;
pub const IP_MULTICAST_IF: u32 = 9u32;
pub const IP_MULTICAST_LOOP: u32 = 11u32;
pub const IP_MULTICAST_TTL: u32 = 10u32;
pub const IP_NRT_INTERFACE: u32 = 74u32;
pub const IP_OPTIONS: u32 = 1u32;
pub const IP_ORIGINAL_ARRIVAL_IF: u32 = 47u32;
pub const IP_PKTINFO: u32 = 19u32;
pub const IP_PKTINFO_EX: u32 = 51u32;
pub const IP_PROTECTION_LEVEL: u32 = 23u32;
pub const IP_RECEIVE_BROADCAST: u32 = 22u32;
pub const IP_RECVDSTADDR: u32 = 25u32;
pub const IP_RECVECN: u32 = 50u32;
pub const IP_RECVERR: u32 = 75u32;
pub const IP_RECVIF: u32 = 24u32;
pub const IP_RECVRTHDR: u32 = 38u32;
pub const IP_RECVTCLASS: u32 = 40u32;
pub const IP_RECVTOS: u32 = 40u32;
pub const IP_RECVTTL: u32 = 21u32;
pub const IP_RTHDR: u32 = 32u32;
pub const IP_TCLASS: u32 = 39u32;
pub const IP_TOS: u32 = 3u32;
pub const IP_TTL: u32 = 4u32;
pub const IP_UNBLOCK_SOURCE: u32 = 18u32;
pub const IP_UNICAST_IF: u32 = 31u32;
pub const IP_UNSPECIFIED_HOP_LIMIT: i32 = -1i32;
pub const IP_UNSPECIFIED_TYPE_OF_SERVICE: i32 = -1i32;
pub const IP_UNSPECIFIED_USER_MTU: u32 = 4294967295u32;
pub const IP_USER_MTU: u32 = 76u32;
pub const IP_WFP_REDIRECT_CONTEXT: u32 = 70u32;
pub const IP_WFP_REDIRECT_RECORDS: u32 = 60u32;
pub const IRDA_PROTO_SOCK_STREAM: u32 = 1u32;
pub const IRLMP_9WIRE_MODE: u32 = 22u32;
pub const IRLMP_DISCOVERY_MODE: u32 = 25u32;
pub const IRLMP_ENUMDEVICES: u32 = 16u32;
pub const IRLMP_EXCLUSIVE_MODE: u32 = 20u32;
pub const IRLMP_IAS_QUERY: u32 = 18u32;
pub const IRLMP_IAS_SET: u32 = 17u32;
pub const IRLMP_IRLPT_MODE: u32 = 21u32;
pub const IRLMP_PARAMETERS: u32 = 24u32;
pub const IRLMP_SEND_PDU_LEN: u32 = 19u32;
pub const IRLMP_SHARP_MODE: u32 = 32u32;
pub const IRLMP_TINYTP_MODE: u32 = 23u32;
pub const ISOPROTO_CLNP: u32 = 31u32;
pub const ISOPROTO_CLTP: u32 = 30u32;
pub const ISOPROTO_ESIS: u32 = 34u32;
pub const ISOPROTO_INACT_NL: u32 = 33u32;
pub const ISOPROTO_INTRAISIS: u32 = 35u32;
pub const ISOPROTO_TP: u32 = 29u32;
pub const ISOPROTO_TP0: u32 = 25u32;
pub const ISOPROTO_TP1: u32 = 26u32;
pub const ISOPROTO_TP2: u32 = 27u32;
pub const ISOPROTO_TP3: u32 = 28u32;
pub const ISOPROTO_TP4: u32 = 29u32;
pub const ISOPROTO_X25: u32 = 32u32;
pub const ISO_EXP_DATA_NUSE: u32 = 1u32;
pub const ISO_EXP_DATA_USE: u32 = 0u32;
pub const ISO_HIERARCHICAL: u32 = 0u32;
pub const ISO_MAX_ADDR_LENGTH: u32 = 64u32;
pub const ISO_NON_HIERARCHICAL: u32 = 1u32;
pub const JL_BOTH: u32 = 4u32;
pub const JL_RECEIVER_ONLY: u32 = 2u32;
pub const JL_SENDER_ONLY: u32 = 1u32;
pub const LAYERED_PROTOCOL: u32 = 0u32;
pub const LITTLEENDIAN: u32 = 1u32;
pub const LM_BAUD_115200: u32 = 115200u32;
pub const LM_BAUD_1152K: u32 = 1152000u32;
pub const LM_BAUD_1200: u32 = 1200u32;
pub const LM_BAUD_16M: u32 = 16000000u32;
pub const LM_BAUD_19200: u32 = 19200u32;
pub const LM_BAUD_2400: u32 = 2400u32;
pub const LM_BAUD_38400: u32 = 38400u32;
pub const LM_BAUD_4M: u32 = 4000000u32;
pub const LM_BAUD_57600: u32 = 57600u32;
pub const LM_BAUD_576K: u32 = 576000u32;
pub const LM_BAUD_9600: u32 = 9600u32;
pub const LM_HB1_Computer: i32 = 4i32;
pub const LM_HB1_Fax: i32 = 32i32;
pub const LM_HB1_LANAccess: i32 = 64i32;
pub const LM_HB1_Modem: i32 = 16i32;
pub const LM_HB1_PDA_Palmtop: i32 = 2i32;
pub const LM_HB1_PnP: i32 = 1i32;
pub const LM_HB1_Printer: i32 = 8i32;
pub const LM_HB2_FileServer: i32 = 2i32;
pub const LM_HB2_Telephony: i32 = 1i32;
pub const LM_HB_Extension: i32 = 128i32;
#[repr(C)]
pub struct LM_IRPARMS(i32);
pub const LOG2_BITS_PER_BYTE: u32 = 3u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPBLOCKINGCALLBACK = unsafe extern "system" fn(dwcontext: usize) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
pub type LPCONDITIONPROC = unsafe extern "system" fn(lpcallerid: *mut WSABUF, lpcallerdata: *mut WSABUF, lpsqos: *mut super::super::NetworkManagement::QoS::QOS, lpgqos: *mut super::super::NetworkManagement::QoS::QOS, lpcalleeid: *mut WSABUF, lpcalleedata: *mut WSABUF, g: *mut u32, dwcallbackdata: usize) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPFN_ACCEPTEX = unsafe extern "system" fn(slistensocket: SOCKET, sacceptsocket: SOCKET, lpoutputbuffer: *mut ::core::ffi::c_void, dwreceivedatalength: u32, dwlocaladdresslength: u32, dwremoteaddresslength: u32, lpdwbytesreceived: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPFN_CONNECTEX = unsafe extern "system" fn(s: SOCKET, name: *const SOCKADDR, namelen: i32, lpsendbuffer: *const ::core::ffi::c_void, dwsenddatalength: u32, lpdwbytessent: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPFN_DISCONNECTEX = unsafe extern "system" fn(s: SOCKET, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, dwflags: u32, dwreserved: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPFN_GETACCEPTEXSOCKADDRS = unsafe extern "system" fn(lpoutputbuffer: *const ::core::ffi::c_void, dwreceivedatalength: u32, dwlocaladdresslength: u32, dwremoteaddresslength: u32, localsockaddr: *mut *mut SOCKADDR, localsockaddrlength: *mut i32, remotesockaddr: *mut *mut SOCKADDR, remotesockaddrlength: *mut i32);
pub type LPFN_NSPAPI = unsafe extern "system" fn() -> u32;
pub type LPFN_RIOCLOSECOMPLETIONQUEUE = unsafe extern "system" fn(cq: *const RIO_CQ_t);
#[cfg(feature = "Win32_Foundation")]
pub type LPFN_RIOCREATECOMPLETIONQUEUE = unsafe extern "system" fn(queuesize: u32, notificationcompletion: *const RIO_NOTIFICATION_COMPLETION) -> *mut RIO_CQ_t;
pub type LPFN_RIOCREATEREQUESTQUEUE = unsafe extern "system" fn(socket: SOCKET, maxoutstandingreceive: u32, maxreceivedatabuffers: u32, maxoutstandingsend: u32, maxsenddatabuffers: u32, receivecq: *const RIO_CQ_t, sendcq: *const RIO_CQ_t, socketcontext: *const ::core::ffi::c_void) -> *mut RIO_RQ_t;
pub type LPFN_RIODEQUEUECOMPLETION = unsafe extern "system" fn(cq: *const RIO_CQ_t, array: *mut RIORESULT, arraysize: u32) -> u32;
pub type LPFN_RIODEREGISTERBUFFER = unsafe extern "system" fn(bufferid: *const RIO_BUFFERID_t);
pub type LPFN_RIONOTIFY = unsafe extern "system" fn(cq: *const RIO_CQ_t) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPFN_RIORECEIVE = unsafe extern "system" fn(socketqueue: *const RIO_RQ_t, pdata: *const RIO_BUF, databuffercount: u32, flags: u32, requestcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
pub type LPFN_RIORECEIVEEX = unsafe extern "system" fn(socketqueue: *const RIO_RQ_t, pdata: *const RIO_BUF, databuffercount: u32, plocaladdress: *const RIO_BUF, premoteaddress: *const RIO_BUF, pcontrolcontext: *const RIO_BUF, pflags: *const RIO_BUF, flags: u32, requestcontext: *const ::core::ffi::c_void) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPFN_RIOREGISTERBUFFER = unsafe extern "system" fn(databuffer: super::super::Foundation::PSTR, datalength: u32) -> *mut RIO_BUFFERID_t;
#[cfg(feature = "Win32_Foundation")]
pub type LPFN_RIORESIZECOMPLETIONQUEUE = unsafe extern "system" fn(cq: *const RIO_CQ_t, queuesize: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPFN_RIORESIZEREQUESTQUEUE = unsafe extern "system" fn(rq: *const RIO_RQ_t, maxoutstandingreceive: u32, maxoutstandingsend: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPFN_RIOSEND = unsafe extern "system" fn(socketqueue: *const RIO_RQ_t, pdata: *const RIO_BUF, databuffercount: u32, flags: u32, requestcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPFN_RIOSENDEX = unsafe extern "system" fn(socketqueue: *const RIO_RQ_t, pdata: *const RIO_BUF, databuffercount: u32, plocaladdress: *const RIO_BUF, premoteaddress: *const RIO_BUF, pcontrolcontext: *const RIO_BUF, pflags: *const RIO_BUF, flags: u32, requestcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPFN_TRANSMITFILE = unsafe extern "system" fn(hsocket: SOCKET, hfile: super::super::Foundation::HANDLE, nnumberofbytestowrite: u32, nnumberofbytespersend: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lptransmitbuffers: *const TRANSMIT_FILE_BUFFERS, dwreserved: u32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPFN_TRANSMITPACKETS = unsafe extern "system" fn(hsocket: SOCKET, lppacketarray: *const TRANSMIT_PACKETS_ELEMENT, nelementcount: u32, nsendsize: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, dwflags: u32) -> super::super::Foundation::BOOL;
pub type LPFN_WSAPOLL = unsafe extern "system" fn(fdarray: *mut WSAPOLLFD, nfds: u32, timeout: i32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPFN_WSARECVMSG = unsafe extern "system" fn(s: SOCKET, lpmsg: *mut WSAMSG, lpdwnumberofbytesrecvd: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPFN_WSASENDMSG = unsafe extern "system" fn(s: SOCKET, lpmsg: *const WSAMSG, dwflags: u32, lpnumberofbytessent: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPLOOKUPSERVICE_COMPLETION_ROUTINE = unsafe extern "system" fn(dwerror: u32, dwbytes: u32, lpoverlapped: *const super::super::System::IO::OVERLAPPED);
pub type LPNSPCLEANUP = unsafe extern "system" fn(lpproviderid: *const ::windows_sys::core::GUID) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPNSPGETSERVICECLASSINFO = unsafe extern "system" fn(lpproviderid: *const ::windows_sys::core::GUID, lpdwbufsize: *const u32, lpserviceclassinfo: *const WSASERVICECLASSINFOW) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPNSPINSTALLSERVICECLASS = unsafe extern "system" fn(lpproviderid: *const ::windows_sys::core::GUID, lpserviceclassinfo: *const WSASERVICECLASSINFOW) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPNSPIOCTL = unsafe extern "system" fn(hlookup: super::super::Foundation::HANDLE, dwcontrolcode: u32, lpvinbuffer: *const ::core::ffi::c_void, cbinbuffer: u32, lpvoutbuffer: *mut ::core::ffi::c_void, cboutbuffer: u32, lpcbbytesreturned: *mut u32, lpcompletion: *const WSACOMPLETION, lpthreadid: *const WSATHREADID) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub type LPNSPLOOKUPSERVICEBEGIN = unsafe extern "system" fn(lpproviderid: *const ::windows_sys::core::GUID, lpqsrestrictions: *const WSAQUERYSETW, lpserviceclassinfo: *const WSASERVICECLASSINFOW, dwcontrolflags: u32, lphlookup: *mut super::super::Foundation::HANDLE) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPNSPLOOKUPSERVICEEND = unsafe extern "system" fn(hlookup: super::super::Foundation::HANDLE) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub type LPNSPLOOKUPSERVICENEXT = unsafe extern "system" fn(hlookup: super::super::Foundation::HANDLE, dwcontrolflags: u32, lpdwbufferlength: *mut u32, lpqsresults: *mut WSAQUERYSETW) -> i32;
pub type LPNSPREMOVESERVICECLASS = unsafe extern "system" fn(lpproviderid: *const ::windows_sys::core::GUID, lpserviceclassid: *const ::windows_sys::core::GUID) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub type LPNSPSETSERVICE = unsafe extern "system" fn(lpproviderid: *const ::windows_sys::core::GUID, lpserviceclassinfo: *const WSASERVICECLASSINFOW, lpqsreginfo: *const WSAQUERYSETW, essoperation: WSAESETSERVICEOP, dwcontrolflags: u32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_IO"))]
pub type LPNSPSTARTUP = unsafe extern "system" fn(lpproviderid: *const ::windows_sys::core::GUID, lpnsproutines: *mut NSP_ROUTINE) -> i32;
pub type LPNSPV2CLEANUP = unsafe extern "system" fn(lpproviderid: *const ::windows_sys::core::GUID, pvclientsessionarg: *const ::core::ffi::c_void) -> i32;
pub type LPNSPV2CLIENTSESSIONRUNDOWN = unsafe extern "system" fn(lpproviderid: *const ::windows_sys::core::GUID, pvclientsessionarg: *const ::core::ffi::c_void);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub type LPNSPV2LOOKUPSERVICEBEGIN = unsafe extern "system" fn(lpproviderid: *const ::windows_sys::core::GUID, lpqsrestrictions: *const WSAQUERYSET2W, dwcontrolflags: u32, lpvclientsessionarg: *const ::core::ffi::c_void, lphlookup: *mut super::super::Foundation::HANDLE) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPNSPV2LOOKUPSERVICEEND = unsafe extern "system" fn(hlookup: super::super::Foundation::HANDLE) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub type LPNSPV2LOOKUPSERVICENEXTEX = unsafe extern "system" fn(hasynccall: super::super::Foundation::HANDLE, hlookup: super::super::Foundation::HANDLE, dwcontrolflags: u32, lpdwbufferlength: *const u32, lpqsresults: *mut WSAQUERYSET2W);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub type LPNSPV2SETSERVICEEX = unsafe extern "system" fn(hasynccall: super::super::Foundation::HANDLE, lpproviderid: *const ::windows_sys::core::GUID, lpqsreginfo: *const WSAQUERYSET2W, essoperation: WSAESETSERVICEOP, dwcontrolflags: u32, lpvclientsessionarg: *const ::core::ffi::c_void);
pub type LPNSPV2STARTUP = unsafe extern "system" fn(lpproviderid: *const ::windows_sys::core::GUID, ppvclientsessionarg: *mut *mut ::core::ffi::c_void) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPSERVICE_CALLBACK_PROC = unsafe extern "system" fn(lparam: super::super::Foundation::LPARAM, hasynctaskhandle: super::super::Foundation::HANDLE);
#[cfg(feature = "Win32_Foundation")]
pub type LPWPUCLOSEEVENT = unsafe extern "system" fn(hevent: super::super::Foundation::HANDLE, lperrno: *mut i32) -> super::super::Foundation::BOOL;
pub type LPWPUCLOSESOCKETHANDLE = unsafe extern "system" fn(s: SOCKET, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWPUCLOSETHREAD = unsafe extern "system" fn(lpthreadid: *const WSATHREADID, lperrno: *mut i32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPWPUCOMPLETEOVERLAPPEDREQUEST = unsafe extern "system" fn(s: SOCKET, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, dwerror: u32, cbtransferred: u32, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWPUCREATEEVENT = unsafe extern "system" fn(lperrno: *mut i32) -> super::super::Foundation::HANDLE;
pub type LPWPUCREATESOCKETHANDLE = unsafe extern "system" fn(dwcatalogentryid: u32, dwcontext: usize, lperrno: *mut i32) -> SOCKET;
pub type LPWPUFDISSET = unsafe extern "system" fn(s: SOCKET, fdset: *const fd_set) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWPUGETPROVIDERPATH = unsafe extern "system" fn(lpproviderid: *const ::windows_sys::core::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpproviderdllpathlen: *mut i32, lperrno: *mut i32) -> i32;
pub type LPWPUMODIFYIFSHANDLE = unsafe extern "system" fn(dwcatalogentryid: u32, proposedhandle: SOCKET, lperrno: *mut i32) -> SOCKET;
#[cfg(feature = "Win32_Foundation")]
pub type LPWPUOPENCURRENTTHREAD = unsafe extern "system" fn(lpthreadid: *mut WSATHREADID, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWPUPOSTMESSAGE = unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPWPUQUERYBLOCKINGCALLBACK = unsafe extern "system" fn(dwcatalogentryid: u32, lplpfncallback: *mut LPBLOCKINGCALLBACK, lpdwcontext: *mut usize, lperrno: *mut i32) -> i32;
pub type LPWPUQUERYSOCKETHANDLECONTEXT = unsafe extern "system" fn(s: SOCKET, lpcontext: *mut usize, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWPUQUEUEAPC = unsafe extern "system" fn(lpthreadid: *const WSATHREADID, lpfnuserapc: LPWSAUSERAPC, dwcontext: usize, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWPURESETEVENT = unsafe extern "system" fn(hevent: super::super::Foundation::HANDLE, lperrno: *mut i32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPWPUSETEVENT = unsafe extern "system" fn(hevent: super::super::Foundation::HANDLE, lperrno: *mut i32) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPWSAOVERLAPPED_COMPLETION_ROUTINE = unsafe extern "system" fn(dwerror: u32, cbtransferred: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, dwflags: u32);
pub type LPWSAUSERAPC = unsafe extern "system" fn(dwcontext: usize);
pub type LPWSCDEINSTALLPROVIDER = unsafe extern "system" fn(lpproviderid: *const ::windows_sys::core::GUID, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSCENABLENSPROVIDER = unsafe extern "system" fn(lpproviderid: *const ::windows_sys::core::GUID, fenable: super::super::Foundation::BOOL) -> i32;
pub type LPWSCENUMPROTOCOLS = unsafe extern "system" fn(lpiprotocols: *const i32, lpprotocolbuffer: *mut WSAPROTOCOL_INFOW, lpdwbufferlength: *mut u32, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSCGETPROVIDERPATH = unsafe extern "system" fn(lpproviderid: *const ::windows_sys::core::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpproviderdllpathlen: *mut i32, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSCINSTALLNAMESPACE = unsafe extern "system" fn(lpszidentifier: super::super::Foundation::PWSTR, lpszpathname: super::super::Foundation::PWSTR, dwnamespace: u32, dwversion: u32, lpproviderid: *const ::windows_sys::core::GUID) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSCINSTALLPROVIDER = unsafe extern "system" fn(lpproviderid: *const ::windows_sys::core::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32;
pub type LPWSCUNINSTALLNAMESPACE = unsafe extern "system" fn(lpproviderid: *const ::windows_sys::core::GUID) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSCUPDATEPROVIDER = unsafe extern "system" fn(lpproviderid: *const ::windows_sys::core::GUID, lpszproviderdllpath: super::super::Foundation::PWSTR, lpprotocolinfolist: *const WSAPROTOCOL_INFOW, dwnumberofentries: u32, lperrno: *mut i32) -> i32;
pub type LPWSCWRITENAMESPACEORDER = unsafe extern "system" fn(lpproviderid: *mut ::windows_sys::core::GUID, dwnumberofentries: u32) -> i32;
pub type LPWSCWRITEPROVIDERORDER = unsafe extern "system" fn(lpwdcatalogentryid: *mut u32, dwnumberofentries: u32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
pub type LPWSPACCEPT = unsafe extern "system" fn(s: SOCKET, addr: *mut SOCKADDR, addrlen: *mut i32, lpfncondition: LPCONDITIONPROC, dwcallbackdata: usize, lperrno: *mut i32) -> SOCKET;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSPADDRESSTOSTRING = unsafe extern "system" fn(lpsaaddress: *const SOCKADDR, dwaddresslength: u32, lpprotocolinfo: *const WSAPROTOCOL_INFOW, lpszaddressstring: super::super::Foundation::PWSTR, lpdwaddressstringlength: *mut u32, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSPASYNCSELECT = unsafe extern "system" fn(s: SOCKET, hwnd: super::super::Foundation::HWND, wmsg: u32, levent: i32, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSPBIND = unsafe extern "system" fn(s: SOCKET, name: *const SOCKADDR, namelen: i32, lperrno: *mut i32) -> i32;
pub type LPWSPCANCELBLOCKINGCALL = unsafe extern "system" fn(lperrno: *mut i32) -> i32;
pub type LPWSPCLEANUP = unsafe extern "system" fn(lperrno: *mut i32) -> i32;
pub type LPWSPCLOSESOCKET = unsafe extern "system" fn(s: SOCKET, lperrno: *mut i32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
pub type LPWSPCONNECT = unsafe extern "system" fn(s: SOCKET, name: *const SOCKADDR, namelen: i32, lpcallerdata: *const WSABUF, lpcalleedata: *mut WSABUF, lpsqos: *const super::super::NetworkManagement::QoS::QOS, lpgqos: *const super::super::NetworkManagement::QoS::QOS, lperrno: *mut i32) -> i32;
pub type LPWSPDUPLICATESOCKET = unsafe extern "system" fn(s: SOCKET, dwprocessid: u32, lpprotocolinfo: *mut WSAPROTOCOL_INFOW, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSPENUMNETWORKEVENTS = unsafe extern "system" fn(s: SOCKET, heventobject: super::super::Foundation::HANDLE, lpnetworkevents: *mut WSANETWORKEVENTS, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSPEVENTSELECT = unsafe extern "system" fn(s: SOCKET, heventobject: super::super::Foundation::HANDLE, lnetworkevents: i32, lperrno: *mut i32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPWSPGETOVERLAPPEDRESULT = unsafe extern "system" fn(s: SOCKET, lpoverlapped: *const super::super::System::IO::OVERLAPPED, lpcbtransfer: *mut u32, fwait: super::super::Foundation::BOOL, lpdwflags: *mut u32, lperrno: *mut i32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSPGETPEERNAME = unsafe extern "system" fn(s: SOCKET, name: *mut SOCKADDR, namelen: *mut i32, lperrno: *mut i32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
pub type LPWSPGETQOSBYNAME = unsafe extern "system" fn(s: SOCKET, lpqosname: *const WSABUF, lpqos: *mut super::super::NetworkManagement::QoS::QOS, lperrno: *mut i32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSPGETSOCKNAME = unsafe extern "system" fn(s: SOCKET, name: *mut SOCKADDR, namelen: *mut i32, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSPGETSOCKOPT = unsafe extern "system" fn(s: SOCKET, level: i32, optname: i32, optval: super::super::Foundation::PSTR, optlen: *mut i32, lperrno: *mut i32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPWSPIOCTL = unsafe extern "system" fn(s: SOCKET, dwiocontrolcode: u32, lpvinbuffer: *const ::core::ffi::c_void, cbinbuffer: u32, lpvoutbuffer: *mut ::core::ffi::c_void, cboutbuffer: u32, lpcbbytesreturned: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE, lpthreadid: *const WSATHREADID, lperrno: *mut i32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS"))]
pub type LPWSPJOINLEAF = unsafe extern "system" fn(s: SOCKET, name: *const SOCKADDR, namelen: i32, lpcallerdata: *const WSABUF, lpcalleedata: *mut WSABUF, lpsqos: *const super::super::NetworkManagement::QoS::QOS, lpgqos: *const super::super::NetworkManagement::QoS::QOS, dwflags: u32, lperrno: *mut i32) -> SOCKET;
pub type LPWSPLISTEN = unsafe extern "system" fn(s: SOCKET, backlog: i32, lperrno: *mut i32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPWSPRECV = unsafe extern "system" fn(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytesrecvd: *mut u32, lpflags: *mut u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE, lpthreadid: *const WSATHREADID, lperrno: *const i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSPRECVDISCONNECT = unsafe extern "system" fn(s: SOCKET, lpinbounddisconnectdata: *const WSABUF, lperrno: *mut i32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPWSPRECVFROM = unsafe extern "system" fn(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytesrecvd: *mut u32, lpflags: *mut u32, lpfrom: *mut SOCKADDR, lpfromlen: *mut i32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE, lpthreadid: *const WSATHREADID, lperrno: *mut i32) -> i32;
pub type LPWSPSELECT = unsafe extern "system" fn(nfds: i32, readfds: *mut fd_set, writefds: *mut fd_set, exceptfds: *mut fd_set, timeout: *const timeval, lperrno: *mut i32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPWSPSEND = unsafe extern "system" fn(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytessent: *mut u32, dwflags: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE, lpthreadid: *const WSATHREADID, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSPSENDDISCONNECT = unsafe extern "system" fn(s: SOCKET, lpoutbounddisconnectdata: *const WSABUF, lperrno: *mut i32) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type LPWSPSENDTO = unsafe extern "system" fn(s: SOCKET, lpbuffers: *const WSABUF, dwbuffercount: u32, lpnumberofbytessent: *mut u32, dwflags: u32, lpto: *const SOCKADDR, itolen: i32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED, lpcompletionroutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE, lpthreadid: *const WSATHREADID, lperrno: *mut i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSPSETSOCKOPT = unsafe extern "system" fn(s: SOCKET, level: i32, optname: i32, optval: super::super::Foundation::PSTR, optlen: i32, lperrno: *mut i32) -> i32;
pub type LPWSPSHUTDOWN = unsafe extern "system" fn(s: SOCKET, how: i32, lperrno: *mut i32) -> i32;
pub type LPWSPSOCKET = unsafe extern "system" fn(af: i32, r#type: i32, protocol: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOW, g: u32, dwflags: u32, lperrno: *mut i32) -> SOCKET;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS", feature = "Win32_System_IO"))]
pub type LPWSPSTARTUP = unsafe extern "system" fn(wversionrequested: u16, lpwspdata: *const WSPData, lpprotocolinfo: *const WSAPROTOCOL_INFOW, upcalltable: WSPUPCALLTABLE, lpproctable: *mut WSPPROC_TABLE) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type LPWSPSTRINGTOADDRESS = unsafe extern "system" fn(addressstring: super::super::Foundation::PWSTR, addressfamily: i32, lpprotocolinfo: *const WSAPROTOCOL_INFOW, lpaddress: *mut SOCKADDR, lpaddresslength: *mut i32, lperrno: *mut i32) -> i32;
pub const LSP_CRYPTO_COMPRESS: u32 = 64u32;
pub const LSP_FIREWALL: u32 = 8u32;
pub const LSP_INBOUND_MODIFY: u32 = 16u32;
pub const LSP_INSPECTOR: u32 = 1u32;
pub const LSP_LOCAL_CACHE: u32 = 128u32;
pub const LSP_OUTBOUND_MODIFY: u32 = 32u32;
pub const LSP_PROXY: u32 = 4u32;
pub const LSP_REDIRECTOR: u32 = 2u32;
pub const LSP_SYSTEM: u32 = 2147483648u32;
pub const LUP_ADDRCONFIG: u32 = 1048576u32;
pub const LUP_API_ANSI: u32 = 16777216u32;
pub const LUP_CONTAINERS: u32 = 2u32;
pub const LUP_DEEP: u32 = 1u32;
pub const LUP_DISABLE_IDN_ENCODING: u32 = 8388608u32;
pub const LUP_DNS_ONLY: u32 = 131072u32;
pub const LUP_DUAL_ADDR: u32 = 2097152u32;
pub const LUP_EXCLUSIVE_CUSTOM_SERVERS: u32 = 134217728u32;
pub const LUP_EXTENDED_QUERYSET: u32 = 33554432u32;
pub const LUP_FILESERVER: u32 = 4194304u32;
pub const LUP_FLUSHCACHE: u32 = 4096u32;
pub const LUP_FLUSHPREVIOUS: u32 = 8192u32;
pub const LUP_FORCE_CLEAR_TEXT: u32 = 1073741824u32;
pub const LUP_NEAREST: u32 = 8u32;
pub const LUP_NOCONTAINERS: u32 = 4u32;
pub const LUP_NON_AUTHORITATIVE: u32 = 16384u32;
pub const LUP_REQUIRE_SECURE: u32 = 268435456u32;
pub const LUP_RESOLUTION_HANDLE: u32 = 2147483648u32;
pub const LUP_RES_SERVICE: u32 = 32768u32;
pub const LUP_RETURN_ADDR: u32 = 256u32;
pub const LUP_RETURN_ALIASES: u32 = 1024u32;
pub const LUP_RETURN_ALL: u32 = 4080u32;
pub const LUP_RETURN_BLOB: u32 = 512u32;
pub const LUP_RETURN_COMMENT: u32 = 128u32;
pub const LUP_RETURN_NAME: u32 = 16u32;
pub const LUP_RETURN_PREFERRED_NAMES: u32 = 65536u32;
pub const LUP_RETURN_QUERY_STRING: u32 = 2048u32;
pub const LUP_RETURN_RESPONSE_FLAGS: u32 = 262144u32;
pub const LUP_RETURN_TTL: u32 = 536870912u32;
pub const LUP_RETURN_TYPE: u32 = 32u32;
pub const LUP_RETURN_VERSION: u32 = 64u32;
pub const LUP_SECURE: u32 = 32768u32;
pub const LUP_SECURE_WITH_FALLBACK: u32 = 67108864u32;
pub const LmCharSetASCII: u32 = 0u32;
pub const LmCharSetISO_8859_1: u32 = 1u32;
pub const LmCharSetISO_8859_2: u32 = 2u32;
pub const LmCharSetISO_8859_3: u32 = 3u32;
pub const LmCharSetISO_8859_4: u32 = 4u32;
pub const LmCharSetISO_8859_5: u32 = 5u32;
pub const LmCharSetISO_8859_6: u32 = 6u32;
pub const LmCharSetISO_8859_7: u32 = 7u32;
pub const LmCharSetISO_8859_8: u32 = 8u32;
pub const LmCharSetISO_8859_9: u32 = 9u32;
pub const LmCharSetUNICODE: u32 = 255u32;
pub const MAXGETHOSTSTRUCT: u32 = 1024u32;
pub const MAX_MCAST_TTL: u32 = 255u32;
pub const MAX_PROTOCOL_CHAIN: u32 = 7u32;
pub const MAX_WINDOW_INCREMENT_PERCENTAGE: u32 = 25u32;
pub const MCAST_BLOCK_SOURCE: u32 = 43u32;
pub const MCAST_JOIN_GROUP: u32 = 41u32;
pub const MCAST_JOIN_SOURCE_GROUP: u32 = 45u32;
pub const MCAST_LEAVE_GROUP: u32 = 42u32;
pub const MCAST_LEAVE_SOURCE_GROUP: u32 = 46u32;
pub const MCAST_UNBLOCK_SOURCE: u32 = 44u32;
pub const MSG_BCAST: u32 = 1024u32;
pub const MSG_CTRUNC: u32 = 512u32;
pub const MSG_ERRQUEUE: u32 = 4096u32;
pub const MSG_INTERRUPT: u32 = 16u32;
pub const MSG_MAXIOVLEN: u32 = 16u32;
pub const MSG_MCAST: u32 = 2048u32;
pub const MSG_PARTIAL: u32 = 32768u32;
pub const MSG_PEEK: u32 = 2u32;
pub const MSG_PUSH_IMMEDIATE: u32 = 32u32;
pub const MSG_TRUNC: u32 = 256u32;
pub const MSG_WAITALL: u32 = 8u32;
#[repr(transparent)]
pub struct MULTICAST_MODE_TYPE(pub i32);
pub const MCAST_INCLUDE: MULTICAST_MODE_TYPE = MULTICAST_MODE_TYPE(0i32);
pub const MCAST_EXCLUDE: MULTICAST_MODE_TYPE = MULTICAST_MODE_TYPE(1i32);
#[repr(C)]
pub struct NAPI_DOMAIN_DESCRIPTION_BLOB(i32);
#[repr(C)]
pub struct NAPI_PROVIDER_INSTALLATION_BLOB(i32);
#[repr(transparent)]
pub struct NAPI_PROVIDER_LEVEL(pub i32);
pub const ProviderLevel_None: NAPI_PROVIDER_LEVEL = NAPI_PROVIDER_LEVEL(0i32);
pub const ProviderLevel_Secondary: NAPI_PROVIDER_LEVEL = NAPI_PROVIDER_LEVEL(1i32);
pub const ProviderLevel_Primary: NAPI_PROVIDER_LEVEL = NAPI_PROVIDER_LEVEL(2i32);
#[repr(transparent)]
pub struct NAPI_PROVIDER_TYPE(pub i32);
pub const ProviderType_Application: NAPI_PROVIDER_TYPE = NAPI_PROVIDER_TYPE(1i32);
pub const ProviderType_Service: NAPI_PROVIDER_TYPE = NAPI_PROVIDER_TYPE(2i32);
pub const NETBIOS_GROUP_NAME: u32 = 1u32;
pub const NETBIOS_NAME_LENGTH: u32 = 16u32;
pub const NETBIOS_TYPE_QUICK_GROUP: u32 = 3u32;
pub const NETBIOS_TYPE_QUICK_UNIQUE: u32 = 2u32;
pub const NETBIOS_UNIQUE_NAME: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NETRESOURCE2A(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NETRESOURCE2W(i32);
pub const NI_DGRAM: u32 = 16u32;
pub const NI_MAXHOST: u32 = 1025u32;
pub const NI_MAXSERV: u32 = 32u32;
pub const NI_NAMEREQD: u32 = 4u32;
pub const NI_NOFQDN: u32 = 1u32;
pub const NI_NUMERICHOST: u32 = 2u32;
pub const NI_NUMERICSERV: u32 = 8u32;
pub const NLA_ALLUSERS_NETWORK: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NLA_BLOB(i32);
#[repr(transparent)]
pub struct NLA_BLOB_DATA_TYPE(pub i32);
pub const NLA_RAW_DATA: NLA_BLOB_DATA_TYPE = NLA_BLOB_DATA_TYPE(0i32);
pub const NLA_INTERFACE: NLA_BLOB_DATA_TYPE = NLA_BLOB_DATA_TYPE(1i32);
pub const NLA_802_1X_LOCATION: NLA_BLOB_DATA_TYPE = NLA_BLOB_DATA_TYPE(2i32);
pub const NLA_CONNECTIVITY: NLA_BLOB_DATA_TYPE = NLA_BLOB_DATA_TYPE(3i32);
pub const NLA_ICS: NLA_BLOB_DATA_TYPE = NLA_BLOB_DATA_TYPE(4i32);
#[repr(transparent)]
pub struct NLA_CONNECTIVITY_TYPE(pub i32);
pub const NLA_NETWORK_AD_HOC: NLA_CONNECTIVITY_TYPE = NLA_CONNECTIVITY_TYPE(0i32);
pub const NLA_NETWORK_MANAGED: NLA_CONNECTIVITY_TYPE = NLA_CONNECTIVITY_TYPE(1i32);
pub const NLA_NETWORK_UNMANAGED: NLA_CONNECTIVITY_TYPE = NLA_CONNECTIVITY_TYPE(2i32);
pub const NLA_NETWORK_UNKNOWN: NLA_CONNECTIVITY_TYPE = NLA_CONNECTIVITY_TYPE(3i32);
pub const NLA_FRIENDLY_NAME: u32 = 2u32;
#[repr(transparent)]
pub struct NLA_INTERNET(pub i32);
pub const NLA_INTERNET_UNKNOWN: NLA_INTERNET = NLA_INTERNET(0i32);
pub const NLA_INTERNET_NO: NLA_INTERNET = NLA_INTERNET(1i32);
pub const NLA_INTERNET_YES: NLA_INTERNET = NLA_INTERNET(2i32);
#[repr(transparent)]
pub struct NL_ADDRESS_TYPE(pub i32);
pub const NlatUnspecified: NL_ADDRESS_TYPE = NL_ADDRESS_TYPE(0i32);
pub const NlatUnicast: NL_ADDRESS_TYPE = NL_ADDRESS_TYPE(1i32);
pub const NlatAnycast: NL_ADDRESS_TYPE = NL_ADDRESS_TYPE(2i32);
pub const NlatMulticast: NL_ADDRESS_TYPE = NL_ADDRESS_TYPE(3i32);
pub const NlatBroadcast: NL_ADDRESS_TYPE = NL_ADDRESS_TYPE(4i32);
pub const NlatInvalid: NL_ADDRESS_TYPE = NL_ADDRESS_TYPE(5i32);
#[repr(transparent)]
pub struct NL_BANDWIDTH_FLAG(pub i32);
pub const NlbwDisabled: NL_BANDWIDTH_FLAG = NL_BANDWIDTH_FLAG(0i32);
pub const NlbwEnabled: NL_BANDWIDTH_FLAG = NL_BANDWIDTH_FLAG(1i32);
pub const NlbwUnchanged: NL_BANDWIDTH_FLAG = NL_BANDWIDTH_FLAG(-1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NL_BANDWIDTH_INFORMATION(i32);
#[repr(transparent)]
pub struct NL_DAD_STATE(pub i32);
pub const NldsInvalid: NL_DAD_STATE = NL_DAD_STATE(0i32);
pub const NldsTentative: NL_DAD_STATE = NL_DAD_STATE(1i32);
pub const NldsDuplicate: NL_DAD_STATE = NL_DAD_STATE(2i32);
pub const NldsDeprecated: NL_DAD_STATE = NL_DAD_STATE(3i32);
pub const NldsPreferred: NL_DAD_STATE = NL_DAD_STATE(4i32);
pub const IpDadStateInvalid: NL_DAD_STATE = NL_DAD_STATE(0i32);
pub const IpDadStateTentative: NL_DAD_STATE = NL_DAD_STATE(1i32);
pub const IpDadStateDuplicate: NL_DAD_STATE = NL_DAD_STATE(2i32);
pub const IpDadStateDeprecated: NL_DAD_STATE = NL_DAD_STATE(3i32);
pub const IpDadStatePreferred: NL_DAD_STATE = NL_DAD_STATE(4i32);
#[repr(transparent)]
pub struct NL_INTERFACE_NETWORK_CATEGORY_STATE(pub i32);
pub const NlincCategoryUnknown: NL_INTERFACE_NETWORK_CATEGORY_STATE = NL_INTERFACE_NETWORK_CATEGORY_STATE(0i32);
pub const NlincPublic: NL_INTERFACE_NETWORK_CATEGORY_STATE = NL_INTERFACE_NETWORK_CATEGORY_STATE(1i32);
pub const NlincPrivate: NL_INTERFACE_NETWORK_CATEGORY_STATE = NL_INTERFACE_NETWORK_CATEGORY_STATE(2i32);
pub const NlincDomainAuthenticated: NL_INTERFACE_NETWORK_CATEGORY_STATE = NL_INTERFACE_NETWORK_CATEGORY_STATE(3i32);
pub const NlincCategoryStateMax: NL_INTERFACE_NETWORK_CATEGORY_STATE = NL_INTERFACE_NETWORK_CATEGORY_STATE(4i32);
#[repr(C)]
pub struct NL_INTERFACE_OFFLOAD_ROD(i32);
#[repr(transparent)]
pub struct NL_LINK_LOCAL_ADDRESS_BEHAVIOR(pub i32);
pub const LinkLocalAlwaysOff: NL_LINK_LOCAL_ADDRESS_BEHAVIOR = NL_LINK_LOCAL_ADDRESS_BEHAVIOR(0i32);
pub const LinkLocalDelayed: NL_LINK_LOCAL_ADDRESS_BEHAVIOR = NL_LINK_LOCAL_ADDRESS_BEHAVIOR(1i32);
pub const LinkLocalAlwaysOn: NL_LINK_LOCAL_ADDRESS_BEHAVIOR = NL_LINK_LOCAL_ADDRESS_BEHAVIOR(2i32);
pub const LinkLocalUnchanged: NL_LINK_LOCAL_ADDRESS_BEHAVIOR = NL_LINK_LOCAL_ADDRESS_BEHAVIOR(-1i32);
#[repr(transparent)]
pub struct NL_NEIGHBOR_STATE(pub i32);
pub const NlnsUnreachable: NL_NEIGHBOR_STATE = NL_NEIGHBOR_STATE(0i32);
pub const NlnsIncomplete: NL_NEIGHBOR_STATE = NL_NEIGHBOR_STATE(1i32);
pub const NlnsProbe: NL_NEIGHBOR_STATE = NL_NEIGHBOR_STATE(2i32);
pub const NlnsDelay: NL_NEIGHBOR_STATE = NL_NEIGHBOR_STATE(3i32);
pub const NlnsStale: NL_NEIGHBOR_STATE = NL_NEIGHBOR_STATE(4i32);
pub const NlnsReachable: NL_NEIGHBOR_STATE = NL_NEIGHBOR_STATE(5i32);
pub const NlnsPermanent: NL_NEIGHBOR_STATE = NL_NEIGHBOR_STATE(6i32);
pub const NlnsMaximum: NL_NEIGHBOR_STATE = NL_NEIGHBOR_STATE(7i32);
#[repr(transparent)]
pub struct NL_NETWORK_CATEGORY(pub i32);
pub const NetworkCategoryPublic: NL_NETWORK_CATEGORY = NL_NETWORK_CATEGORY(0i32);
pub const NetworkCategoryPrivate: NL_NETWORK_CATEGORY = NL_NETWORK_CATEGORY(1i32);
pub const NetworkCategoryDomainAuthenticated: NL_NETWORK_CATEGORY = NL_NETWORK_CATEGORY(2i32);
pub const NetworkCategoryUnchanged: NL_NETWORK_CATEGORY = NL_NETWORK_CATEGORY(-1i32);
pub const NetworkCategoryUnknown: NL_NETWORK_CATEGORY = NL_NETWORK_CATEGORY(-1i32);
#[repr(transparent)]
pub struct NL_NETWORK_CONNECTIVITY_COST_HINT(pub i32);
pub const NetworkConnectivityCostHintUnknown: NL_NETWORK_CONNECTIVITY_COST_HINT = NL_NETWORK_CONNECTIVITY_COST_HINT(0i32);
pub const NetworkConnectivityCostHintUnrestricted: NL_NETWORK_CONNECTIVITY_COST_HINT = NL_NETWORK_CONNECTIVITY_COST_HINT(1i32);
pub const NetworkConnectivityCostHintFixed: NL_NETWORK_CONNECTIVITY_COST_HINT = NL_NETWORK_CONNECTIVITY_COST_HINT(2i32);
pub const NetworkConnectivityCostHintVariable: NL_NETWORK_CONNECTIVITY_COST_HINT = NL_NETWORK_CONNECTIVITY_COST_HINT(3i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NL_NETWORK_CONNECTIVITY_HINT(i32);
#[repr(transparent)]
pub struct NL_NETWORK_CONNECTIVITY_LEVEL_HINT(pub i32);
pub const NetworkConnectivityLevelHintUnknown: NL_NETWORK_CONNECTIVITY_LEVEL_HINT = NL_NETWORK_CONNECTIVITY_LEVEL_HINT(0i32);
pub const NetworkConnectivityLevelHintNone: NL_NETWORK_CONNECTIVITY_LEVEL_HINT = NL_NETWORK_CONNECTIVITY_LEVEL_HINT(1i32);
pub const NetworkConnectivityLevelHintLocalAccess: NL_NETWORK_CONNECTIVITY_LEVEL_HINT = NL_NETWORK_CONNECTIVITY_LEVEL_HINT(2i32);
pub const NetworkConnectivityLevelHintInternetAccess: NL_NETWORK_CONNECTIVITY_LEVEL_HINT = NL_NETWORK_CONNECTIVITY_LEVEL_HINT(3i32);
pub const NetworkConnectivityLevelHintConstrainedInternetAccess: NL_NETWORK_CONNECTIVITY_LEVEL_HINT = NL_NETWORK_CONNECTIVITY_LEVEL_HINT(4i32);
pub const NetworkConnectivityLevelHintHidden: NL_NETWORK_CONNECTIVITY_LEVEL_HINT = NL_NETWORK_CONNECTIVITY_LEVEL_HINT(5i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NL_PATH_BANDWIDTH_ROD(i32);
#[repr(transparent)]
pub struct NL_PREFIX_ORIGIN(pub i32);
pub const IpPrefixOriginOther: NL_PREFIX_ORIGIN = NL_PREFIX_ORIGIN(0i32);
pub const IpPrefixOriginManual: NL_PREFIX_ORIGIN = NL_PREFIX_ORIGIN(1i32);
pub const IpPrefixOriginWellKnown: NL_PREFIX_ORIGIN = NL_PREFIX_ORIGIN(2i32);
pub const IpPrefixOriginDhcp: NL_PREFIX_ORIGIN = NL_PREFIX_ORIGIN(3i32);
pub const IpPrefixOriginRouterAdvertisement: NL_PREFIX_ORIGIN = NL_PREFIX_ORIGIN(4i32);
pub const IpPrefixOriginUnchanged: NL_PREFIX_ORIGIN = NL_PREFIX_ORIGIN(16i32);
#[repr(transparent)]
pub struct NL_ROUTER_DISCOVERY_BEHAVIOR(pub i32);
pub const RouterDiscoveryDisabled: NL_ROUTER_DISCOVERY_BEHAVIOR = NL_ROUTER_DISCOVERY_BEHAVIOR(0i32);
pub const RouterDiscoveryEnabled: NL_ROUTER_DISCOVERY_BEHAVIOR = NL_ROUTER_DISCOVERY_BEHAVIOR(1i32);
pub const RouterDiscoveryDhcp: NL_ROUTER_DISCOVERY_BEHAVIOR = NL_ROUTER_DISCOVERY_BEHAVIOR(2i32);
pub const RouterDiscoveryUnchanged: NL_ROUTER_DISCOVERY_BEHAVIOR = NL_ROUTER_DISCOVERY_BEHAVIOR(-1i32);
#[repr(transparent)]
pub struct NL_ROUTE_ORIGIN(pub i32);
pub const NlroManual: NL_ROUTE_ORIGIN = NL_ROUTE_ORIGIN(0i32);
pub const NlroWellKnown: NL_ROUTE_ORIGIN = NL_ROUTE_ORIGIN(1i32);
pub const NlroDHCP: NL_ROUTE_ORIGIN = NL_ROUTE_ORIGIN(2i32);
pub const NlroRouterAdvertisement: NL_ROUTE_ORIGIN = NL_ROUTE_ORIGIN(3i32);
pub const Nlro6to4: NL_ROUTE_ORIGIN = NL_ROUTE_ORIGIN(4i32);
#[repr(transparent)]
pub struct NL_ROUTE_PROTOCOL(pub i32);
pub const RouteProtocolOther: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(1i32);
pub const RouteProtocolLocal: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(2i32);
pub const RouteProtocolNetMgmt: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(3i32);
pub const RouteProtocolIcmp: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(4i32);
pub const RouteProtocolEgp: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(5i32);
pub const RouteProtocolGgp: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(6i32);
pub const RouteProtocolHello: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(7i32);
pub const RouteProtocolRip: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(8i32);
pub const RouteProtocolIsIs: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(9i32);
pub const RouteProtocolEsIs: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10i32);
pub const RouteProtocolCisco: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(11i32);
pub const RouteProtocolBbn: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(12i32);
pub const RouteProtocolOspf: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(13i32);
pub const RouteProtocolBgp: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(14i32);
pub const RouteProtocolIdpr: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(15i32);
pub const RouteProtocolEigrp: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(16i32);
pub const RouteProtocolDvmrp: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(17i32);
pub const RouteProtocolRpl: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(18i32);
pub const RouteProtocolDhcp: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(19i32);
pub const MIB_IPPROTO_OTHER: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(1i32);
pub const PROTO_IP_OTHER: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(1i32);
pub const MIB_IPPROTO_LOCAL: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(2i32);
pub const PROTO_IP_LOCAL: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(2i32);
pub const MIB_IPPROTO_NETMGMT: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(3i32);
pub const PROTO_IP_NETMGMT: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(3i32);
pub const MIB_IPPROTO_ICMP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(4i32);
pub const PROTO_IP_ICMP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(4i32);
pub const MIB_IPPROTO_EGP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(5i32);
pub const PROTO_IP_EGP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(5i32);
pub const MIB_IPPROTO_GGP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(6i32);
pub const PROTO_IP_GGP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(6i32);
pub const MIB_IPPROTO_HELLO: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(7i32);
pub const PROTO_IP_HELLO: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(7i32);
pub const MIB_IPPROTO_RIP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(8i32);
pub const PROTO_IP_RIP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(8i32);
pub const MIB_IPPROTO_IS_IS: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(9i32);
pub const PROTO_IP_IS_IS: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(9i32);
pub const MIB_IPPROTO_ES_IS: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10i32);
pub const PROTO_IP_ES_IS: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10i32);
pub const MIB_IPPROTO_CISCO: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(11i32);
pub const PROTO_IP_CISCO: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(11i32);
pub const MIB_IPPROTO_BBN: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(12i32);
pub const PROTO_IP_BBN: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(12i32);
pub const MIB_IPPROTO_OSPF: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(13i32);
pub const PROTO_IP_OSPF: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(13i32);
pub const MIB_IPPROTO_BGP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(14i32);
pub const PROTO_IP_BGP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(14i32);
pub const MIB_IPPROTO_IDPR: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(15i32);
pub const PROTO_IP_IDPR: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(15i32);
pub const MIB_IPPROTO_EIGRP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(16i32);
pub const PROTO_IP_EIGRP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(16i32);
pub const MIB_IPPROTO_DVMRP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(17i32);
pub const PROTO_IP_DVMRP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(17i32);
pub const MIB_IPPROTO_RPL: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(18i32);
pub const PROTO_IP_RPL: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(18i32);
pub const MIB_IPPROTO_DHCP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(19i32);
pub const PROTO_IP_DHCP: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(19i32);
pub const MIB_IPPROTO_NT_AUTOSTATIC: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10002i32);
pub const PROTO_IP_NT_AUTOSTATIC: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10002i32);
pub const MIB_IPPROTO_NT_STATIC: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10006i32);
pub const PROTO_IP_NT_STATIC: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10006i32);
pub const MIB_IPPROTO_NT_STATIC_NON_DOD: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10007i32);
pub const PROTO_IP_NT_STATIC_NON_DOD: NL_ROUTE_PROTOCOL = NL_ROUTE_PROTOCOL(10007i32);
#[repr(transparent)]
pub struct NL_SUFFIX_ORIGIN(pub i32);
pub const NlsoOther: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(0i32);
pub const NlsoManual: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(1i32);
pub const NlsoWellKnown: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(2i32);
pub const NlsoDhcp: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(3i32);
pub const NlsoLinkLayerAddress: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(4i32);
pub const NlsoRandom: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(5i32);
pub const IpSuffixOriginOther: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(0i32);
pub const IpSuffixOriginManual: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(1i32);
pub const IpSuffixOriginWellKnown: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(2i32);
pub const IpSuffixOriginDhcp: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(3i32);
pub const IpSuffixOriginLinkLayerAddress: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(4i32);
pub const IpSuffixOriginRandom: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(5i32);
pub const IpSuffixOriginUnchanged: NL_SUFFIX_ORIGIN = NL_SUFFIX_ORIGIN(16i32);
pub const NSPROTO_IPX: u32 = 1000u32;
pub const NSPROTO_SPX: u32 = 1256u32;
pub const NSPROTO_SPXII: u32 = 1257u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct NSPV2_ROUTINE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_IO"))]
#[repr(C)]
pub struct NSP_ROUTINE(i32);
pub const NSTYPE_DYNAMIC: u32 = 2u32;
pub const NSTYPE_ENUMERABLE: u32 = 4u32;
pub const NSTYPE_HIERARCHICAL: u32 = 1u32;
pub const NSTYPE_WORKGROUP: u32 = 8u32;
pub const NS_ALL: u32 = 0u32;
pub const NS_DEFAULT: u32 = 0u32;
pub const NS_DHCP: u32 = 6u32;
pub const NS_DNS: u32 = 12u32;
pub const NS_EMAIL: u32 = 37u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NS_INFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NS_INFOW(i32);
pub const NS_LOCALNAME: u32 = 19u32;
pub const NS_MS: u32 = 30u32;
pub const NS_NBP: u32 = 20u32;
pub const NS_NDS: u32 = 2u32;
pub const NS_NETBT: u32 = 13u32;
pub const NS_NETDES: u32 = 60u32;
pub const NS_NIS: u32 = 41u32;
pub const NS_NISPLUS: u32 = 42u32;
pub const NS_NLA: u32 = 15u32;
pub const NS_NTDS: u32 = 32u32;
pub const NS_PEER_BROWSE: u32 = 3u32;
pub const NS_SAP: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct NS_SERVICE_INFOA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct NS_SERVICE_INFOW(i32);
pub const NS_SLP: u32 = 5u32;
pub const NS_STDA: u32 = 31u32;
pub const NS_TCPIP_HOSTS: u32 = 11u32;
pub const NS_TCPIP_LOCAL: u32 = 10u32;
pub const NS_VNS: u32 = 50u32;
pub const NS_WINS: u32 = 14u32;
pub const NS_WRQ: u32 = 50u32;
pub const NS_X500: u32 = 40u32;
pub const PFL_HIDDEN: u32 = 4u32;
pub const PFL_MATCHES_PROTOCOL_ZERO: u32 = 8u32;
pub const PFL_MULTIPLE_PROTO_ENTRIES: u32 = 1u32;
pub const PFL_NETWORKDIRECT_PROVIDER: u32 = 16u32;
pub const PFL_RECOMMENDED_PROTO_ENTRY: u32 = 2u32;
pub const PF_APPLETALK: u16 = 16u16;
pub const PF_ATM: u16 = 22u16;
pub const PF_BAN: u16 = 21u16;
pub const PF_CCITT: u16 = 10u16;
pub const PF_CHAOS: u16 = 5u16;
pub const PF_DATAKIT: u16 = 9u16;
pub const PF_DECnet: u16 = 12u16;
pub const PF_DLI: u16 = 13u16;
pub const PF_ECMA: u16 = 8u16;
pub const PF_FIREFOX: u16 = 19u16;
pub const PF_HYLINK: u16 = 15u16;
pub const PF_IMPLINK: u16 = 3u16;
pub const PF_IPX: u16 = 6u16;
pub const PF_IRDA: u16 = 26u16;
pub const PF_ISO: u16 = 7u16;
pub const PF_LAT: u16 = 14u16;
pub const PF_MAX: u16 = 29u16;
pub const PF_NS: u16 = 6u16;
pub const PF_OSI: u16 = 7u16;
pub const PF_PUP: u16 = 4u16;
pub const PF_SNA: u16 = 11u16;
pub const PF_UNIX: u16 = 1u16;
pub const PF_UNKNOWN1: u16 = 20u16;
pub const PF_VOICEVIEW: u16 = 18u16;
pub const PI_ALLOWED: u32 = 0u32;
pub const PI_NUMBER_NOT_AVAILABLE: u32 = 128u32;
pub const PI_RESTRICTED: u32 = 64u32;
#[repr(transparent)]
pub struct PMTUD_STATE(pub i32);
pub const IP_PMTUDISC_NOT_SET: PMTUD_STATE = PMTUD_STATE(0i32);
pub const IP_PMTUDISC_DO: PMTUD_STATE = PMTUD_STATE(1i32);
pub const IP_PMTUDISC_DONT: PMTUD_STATE = PMTUD_STATE(2i32);
pub const IP_PMTUDISC_PROBE: PMTUD_STATE = PMTUD_STATE(3i32);
pub const IP_PMTUDISC_MAX: PMTUD_STATE = PMTUD_STATE(4i32);
pub const POLLERR: u32 = 1u32;
pub const POLLHUP: u32 = 2u32;
pub const POLLNVAL: u32 = 4u32;
pub const POLLOUT: u32 = 16u32;
pub const POLLPRI: u32 = 1024u32;
pub const POLLRDBAND: u32 = 512u32;
pub const POLLRDNORM: u32 = 256u32;
pub const POLLWRBAND: u32 = 32u32;
pub const POLLWRNORM: u32 = 16u32;
#[repr(C)]
pub struct PRIORITY_STATUS(i32);
pub const PROP_ADDRESSES: u32 = 256u32;
pub const PROP_ALL: u32 = 2147483648u32;
pub const PROP_COMMENT: u32 = 1u32;
pub const PROP_DISPLAY_HINT: u32 = 4u32;
pub const PROP_LOCALE: u32 = 2u32;
pub const PROP_MACHINE: u32 = 32u32;
pub const PROP_SD: u32 = 512u32;
pub const PROP_START_TIME: u32 = 16u32;
pub const PROP_VERSION: u32 = 8u32;
pub const PROTECTION_LEVEL_DEFAULT: u32 = 20u32;
pub const PROTECTION_LEVEL_EDGERESTRICTED: u32 = 20u32;
pub const PROTECTION_LEVEL_RESTRICTED: u32 = 30u32;
pub const PROTECTION_LEVEL_UNRESTRICTED: u32 = 10u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PROTOCOL_INFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PROTOCOL_INFOW(i32);
pub const PVD_CONFIG: u32 = 12289u32;
#[repr(C)]
pub struct Q2931_IE(i32);
#[repr(transparent)]
pub struct Q2931_IE_TYPE(pub i32);
pub const IE_AALParameters: Q2931_IE_TYPE = Q2931_IE_TYPE(0i32);
pub const IE_TrafficDescriptor: Q2931_IE_TYPE = Q2931_IE_TYPE(1i32);
pub const IE_BroadbandBearerCapability: Q2931_IE_TYPE = Q2931_IE_TYPE(2i32);
pub const IE_BHLI: Q2931_IE_TYPE = Q2931_IE_TYPE(3i32);
pub const IE_BLLI: Q2931_IE_TYPE = Q2931_IE_TYPE(4i32);
pub const IE_CalledPartyNumber: Q2931_IE_TYPE = Q2931_IE_TYPE(5i32);
pub const IE_CalledPartySubaddress: Q2931_IE_TYPE = Q2931_IE_TYPE(6i32);
pub const IE_CallingPartyNumber: Q2931_IE_TYPE = Q2931_IE_TYPE(7i32);
pub const IE_CallingPartySubaddress: Q2931_IE_TYPE = Q2931_IE_TYPE(8i32);
pub const IE_Cause: Q2931_IE_TYPE = Q2931_IE_TYPE(9i32);
pub const IE_QOSClass: Q2931_IE_TYPE = Q2931_IE_TYPE(10i32);
pub const IE_TransitNetworkSelection: Q2931_IE_TYPE = Q2931_IE_TYPE(11i32);
pub const QOS_CLASS0: u32 = 0u32;
pub const QOS_CLASS1: u32 = 1u32;
pub const QOS_CLASS2: u32 = 2u32;
pub const QOS_CLASS3: u32 = 3u32;
pub const QOS_CLASS4: u32 = 4u32;
#[repr(C)]
pub struct RCVALL_IF(i32);
#[repr(transparent)]
pub struct RCVALL_VALUE(pub i32);
pub const RCVALL_OFF: RCVALL_VALUE = RCVALL_VALUE(0i32);
pub const RCVALL_ON: RCVALL_VALUE = RCVALL_VALUE(1i32);
pub const RCVALL_SOCKETLEVELONLY: RCVALL_VALUE = RCVALL_VALUE(2i32);
pub const RCVALL_IPLEVEL: RCVALL_VALUE = RCVALL_VALUE(3i32);
pub const REAL_TIME_NOTIFICATION_CAPABILITY: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1801027994, data2: 23726, data3: 18733, data4: [169, 1, 42, 60, 44, 80, 22, 79] };
pub const REAL_TIME_NOTIFICATION_CAPABILITY_EX: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1749277187, data2: 5450, data3: 17942, data4: [165, 8, 68, 55, 18, 149, 249, 107] };
#[repr(C)]
pub struct REAL_TIME_NOTIFICATION_SETTING_INPUT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct REAL_TIME_NOTIFICATION_SETTING_INPUT_EX(i32);
#[repr(C)]
pub struct REAL_TIME_NOTIFICATION_SETTING_OUTPUT(i32);
#[repr(transparent)]
pub struct RESOURCE_DISPLAY_TYPE(pub u32);
pub const RESOURCEDISPLAYTYPE_DOMAIN: RESOURCE_DISPLAY_TYPE = RESOURCE_DISPLAY_TYPE(1u32);
pub const RESOURCEDISPLAYTYPE_FILE: RESOURCE_DISPLAY_TYPE = RESOURCE_DISPLAY_TYPE(4u32);
pub const RESOURCEDISPLAYTYPE_GENERIC: RESOURCE_DISPLAY_TYPE = RESOURCE_DISPLAY_TYPE(0u32);
pub const RESOURCEDISPLAYTYPE_GROUP: RESOURCE_DISPLAY_TYPE = RESOURCE_DISPLAY_TYPE(5u32);
pub const RESOURCEDISPLAYTYPE_SERVER: RESOURCE_DISPLAY_TYPE = RESOURCE_DISPLAY_TYPE(2u32);
pub const RESOURCEDISPLAYTYPE_SHARE: RESOURCE_DISPLAY_TYPE = RESOURCE_DISPLAY_TYPE(3u32);
pub const RESOURCEDISPLAYTYPE_TREE: RESOURCE_DISPLAY_TYPE = RESOURCE_DISPLAY_TYPE(10u32);
pub const RESULT_IS_ADDED: u32 = 16u32;
pub const RESULT_IS_ALIAS: u32 = 1u32;
pub const RESULT_IS_CHANGED: u32 = 32u32;
pub const RESULT_IS_DELETED: u32 = 64u32;
pub const RES_FIND_MULTIPLE: u32 = 2u32;
pub const RES_FLUSH_CACHE: u32 = 2u32;
pub const RES_SERVICE: u32 = 4u32;
pub const RES_SOFT_SEARCH: u32 = 1u32;
pub const RES_UNUSED_1: u32 = 1u32;
#[repr(C)]
pub struct RIORESULT(i32);
#[repr(C)]
pub struct RIO_BUF(i32);
#[repr(C)]
pub struct RIO_BUFFERID_t(i32);
#[repr(C)]
pub struct RIO_CMSG_BUFFER(i32);
pub const RIO_CORRUPT_CQ: u32 = 4294967295u32;
#[repr(C)]
pub struct RIO_CQ_t(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct RIO_EXTENSION_FUNCTION_TABLE(i32);
pub const RIO_MAX_CQ_SIZE: u32 = 134217728u32;
pub const RIO_MSG_COMMIT_ONLY: u32 = 8u32;
pub const RIO_MSG_DEFER: u32 = 2u32;
pub const RIO_MSG_DONT_NOTIFY: u32 = 1u32;
pub const RIO_MSG_WAITALL: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct RIO_NOTIFICATION_COMPLETION(i32);
#[repr(transparent)]
pub struct RIO_NOTIFICATION_COMPLETION_TYPE(pub i32);
pub const RIO_EVENT_COMPLETION: RIO_NOTIFICATION_COMPLETION_TYPE = RIO_NOTIFICATION_COMPLETION_TYPE(1i32);
pub const RIO_IOCP_COMPLETION: RIO_NOTIFICATION_COMPLETION_TYPE = RIO_NOTIFICATION_COMPLETION_TYPE(2i32);
#[repr(C)]
pub struct RIO_RQ_t(i32);
pub const RM_ADD_RECEIVE_IF: u32 = 1008u32;
pub const RM_DEL_RECEIVE_IF: u32 = 1009u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct RM_FEC_INFO(i32);
pub const RM_FLUSHCACHE: u32 = 1003u32;
pub const RM_HIGH_SPEED_INTRANET_OPT: u32 = 1014u32;
pub const RM_LATEJOIN: u32 = 1006u32;
pub const RM_OPTIONSBASE: u32 = 1000u32;
pub const RM_RATE_WINDOW_SIZE: u32 = 1001u32;
pub const RM_RECEIVER_STATISTICS: u32 = 1013u32;
#[repr(C)]
pub struct RM_RECEIVER_STATS(i32);
pub const RM_SENDER_STATISTICS: u32 = 1005u32;
#[repr(C)]
pub struct RM_SENDER_STATS(i32);
pub const RM_SENDER_WINDOW_ADVANCE_METHOD: u32 = 1004u32;
#[repr(C)]
pub struct RM_SEND_WINDOW(i32);
pub const RM_SEND_WINDOW_ADV_RATE: u32 = 1010u32;
pub const RM_SET_MCAST_TTL: u32 = 1012u32;
pub const RM_SET_MESSAGE_BOUNDARY: u32 = 1002u32;
pub const RM_SET_SEND_IF: u32 = 1007u32;
pub const RM_USE_FEC: u32 = 1011u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct RSS_SCALABILITY_INFO(i32);
pub const SAP_FIELD_ABSENT: u32 = 4294967294u32;
pub const SAP_FIELD_ANY: u32 = 4294967295u32;
pub const SAP_FIELD_ANY_AESA_REST: u32 = 4294967291u32;
pub const SAP_FIELD_ANY_AESA_SEL: u32 = 4294967290u32;
#[repr(C)]
pub struct SCOPE_ID(i32);
#[repr(transparent)]
pub struct SCOPE_LEVEL(pub i32);
pub const ScopeLevelInterface: SCOPE_LEVEL = SCOPE_LEVEL(1i32);
pub const ScopeLevelLink: SCOPE_LEVEL = SCOPE_LEVEL(2i32);
pub const ScopeLevelSubnet: SCOPE_LEVEL = SCOPE_LEVEL(3i32);
pub const ScopeLevelAdmin: SCOPE_LEVEL = SCOPE_LEVEL(4i32);
pub const ScopeLevelSite: SCOPE_LEVEL = SCOPE_LEVEL(5i32);
pub const ScopeLevelOrganization: SCOPE_LEVEL = SCOPE_LEVEL(8i32);
pub const ScopeLevelGlobal: SCOPE_LEVEL = SCOPE_LEVEL(14i32);
pub const ScopeLevelCount: SCOPE_LEVEL = SCOPE_LEVEL(16i32);
pub const SD_BOTH: u32 = 2u32;
pub const SD_RECEIVE: u32 = 0u32;
pub const SD_SEND: u32 = 1u32;
pub const SECURITY_PROTOCOL_NONE: u32 = 0u32;
pub const SENDER_DEFAULT_LATE_JOINER_PERCENTAGE: u32 = 0u32;
pub const SENDER_DEFAULT_RATE_KBITS_PER_SEC: u32 = 56u32;
pub const SENDER_DEFAULT_WINDOW_ADV_PERCENTAGE: u32 = 15u32;
pub const SENDER_MAX_LATE_JOINER_PERCENTAGE: u32 = 75u32;
#[repr(transparent)]
pub struct SEND_FLAGS(pub u32);
pub const MSG_DONTROUTE: SEND_FLAGS = SEND_FLAGS(4u32);
pub const MSG_OOB: SEND_FLAGS = SEND_FLAGS(1u32);
#[repr(C)]
pub struct SERVICE_ADDRESS(i32);
#[repr(C)]
pub struct SERVICE_ADDRESSES(i32);
pub const SERVICE_ADDRESS_FLAG_RPC_CN: u32 = 1u32;
pub const SERVICE_ADDRESS_FLAG_RPC_DG: u32 = 2u32;
pub const SERVICE_ADDRESS_FLAG_RPC_NB: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVICE_ASYNC_INFO(i32);
pub const SERVICE_FLAG_DEFER: u32 = 1u32;
pub const SERVICE_FLAG_HARD: u32 = 2u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct SERVICE_INFOA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct SERVICE_INFOW(i32);
pub const SERVICE_LOCAL: u32 = 4u32;
pub const SERVICE_MULTIPLE: u32 = 1u32;
pub const SERVICE_RESOURCE: u32 = 1u32;
pub const SERVICE_SERVICE: u32 = 2u32;
#[repr(C)]
pub struct SERVICE_TYPE_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVICE_TYPE_INFO_ABSA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVICE_TYPE_INFO_ABSW(i32);
#[repr(C)]
pub struct SERVICE_TYPE_VALUE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVICE_TYPE_VALUE_ABSA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SERVICE_TYPE_VALUE_ABSW(i32);
#[repr(transparent)]
pub struct SET_SERVICE_OPERATION(pub u32);
pub const SERVICE_REGISTER: SET_SERVICE_OPERATION = SET_SERVICE_OPERATION(1u32);
pub const SERVICE_DEREGISTER: SET_SERVICE_OPERATION = SET_SERVICE_OPERATION(2u32);
pub const SERVICE_FLUSH: SET_SERVICE_OPERATION = SET_SERVICE_OPERATION(3u32);
pub const SERVICE_ADD_TYPE: SET_SERVICE_OPERATION = SET_SERVICE_OPERATION(4u32);
pub const SERVICE_DELETE_TYPE: SET_SERVICE_OPERATION = SET_SERVICE_OPERATION(5u32);
pub const SET_SERVICE_PARTIAL_SUCCESS: u32 = 1u32;
pub const SG_CONSTRAINED_GROUP: u32 = 2u32;
pub const SG_UNCONSTRAINED_GROUP: u32 = 1u32;
pub const SIOCATMARK: i32 = 1074033415i32;
pub const SIOCGHIWAT: i32 = 1074033409i32;
pub const SIOCGLOWAT: i32 = 1074033411i32;
pub const SIOCSHIWAT: i32 = -2147192064i32;
pub const SIOCSLOWAT: i32 = -2147192062i32;
pub const SIO_ASSOCIATE_PVC: u32 = 2417360899u32;
pub const SIO_GET_ATM_ADDRESS: u32 = 3491102722u32;
pub const SIO_GET_ATM_CONNECTION_ID: u32 = 1343619076u32;
pub const SIO_GET_NUMBER_OF_ATM_DEVICES: u32 = 1343619073u32;
pub const SI_NETWORK: u32 = 3u32;
pub const SI_USER_FAILED: u32 = 2u32;
pub const SI_USER_NOT_SCREENED: u32 = 0u32;
pub const SI_USER_PASSED: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SOCKADDR(i32);
#[repr(C)]
pub struct SOCKADDR_DL(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SOCKADDR_IN(i32);
#[repr(C)]
pub struct SOCKADDR_IN6(i32);
#[repr(C)]
pub struct SOCKADDR_IN6_PAIR(i32);
#[repr(C)]
pub struct SOCKADDR_IN6_W2KSP1(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SOCKADDR_INET(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SOCKADDR_IRDA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SOCKADDR_STORAGE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SOCKADDR_STORAGE_XP(i32);
#[repr(C)]
pub struct SOCKET(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SOCKET_ADDRESS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SOCKET_ADDRESS_LIST(i32);
pub const SOCKET_DEFAULT2_QM_POLICY: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2932010908, data2: 14925, data3: 19774, data4: [136, 66, 35, 153, 66, 227, 154, 71] };
pub const SOCKET_ERROR: i32 = -1i32;
pub const SOCKET_INFO_CONNECTION_ENCRYPTED: u32 = 2u32;
pub const SOCKET_INFO_CONNECTION_IMPERSONATED: u32 = 4u32;
pub const SOCKET_INFO_CONNECTION_SECURED: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SOCKET_PEER_TARGET_NAME(i32);
#[repr(transparent)]
pub struct SOCKET_PRIORITY_HINT(pub i32);
pub const SocketPriorityHintVeryLow: SOCKET_PRIORITY_HINT = SOCKET_PRIORITY_HINT(0i32);
pub const SocketPriorityHintLow: SOCKET_PRIORITY_HINT = SOCKET_PRIORITY_HINT(1i32);
pub const SocketPriorityHintNormal: SOCKET_PRIORITY_HINT = SOCKET_PRIORITY_HINT(2i32);
pub const SocketMaximumPriorityHintType: SOCKET_PRIORITY_HINT = SOCKET_PRIORITY_HINT(3i32);
#[cfg(feature = "Win32_System_Kernel")]
#[repr(C)]
pub struct SOCKET_PROCESSOR_AFFINITY(i32);
pub const SOCKET_QUERY_IPSEC2_ABORT_CONNECTION_ON_FIELD_CHANGE: u32 = 1u32;
pub const SOCKET_QUERY_IPSEC2_FIELD_MASK_MM_SA_ID: u32 = 1u32;
pub const SOCKET_QUERY_IPSEC2_FIELD_MASK_QM_SA_ID: u32 = 2u32;
#[repr(transparent)]
pub struct SOCKET_SECURITY_PROTOCOL(pub i32);
pub const SOCKET_SECURITY_PROTOCOL_DEFAULT: SOCKET_SECURITY_PROTOCOL = SOCKET_SECURITY_PROTOCOL(0i32);
pub const SOCKET_SECURITY_PROTOCOL_IPSEC: SOCKET_SECURITY_PROTOCOL = SOCKET_SECURITY_PROTOCOL(1i32);
pub const SOCKET_SECURITY_PROTOCOL_IPSEC2: SOCKET_SECURITY_PROTOCOL = SOCKET_SECURITY_PROTOCOL(2i32);
pub const SOCKET_SECURITY_PROTOCOL_INVALID: SOCKET_SECURITY_PROTOCOL = SOCKET_SECURITY_PROTOCOL(3i32);
#[repr(C)]
pub struct SOCKET_SECURITY_QUERY_INFO(i32);
#[repr(C)]
pub struct SOCKET_SECURITY_QUERY_INFO_IPSEC2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SOCKET_SECURITY_QUERY_TEMPLATE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SOCKET_SECURITY_QUERY_TEMPLATE_IPSEC2(i32);
#[repr(C)]
pub struct SOCKET_SECURITY_SETTINGS(i32);
#[repr(C)]
pub struct SOCKET_SECURITY_SETTINGS_IPSEC(i32);
pub const SOCKET_SETTINGS_ALLOW_INSECURE: u32 = 2u32;
pub const SOCKET_SETTINGS_GUARANTEE_ENCRYPTION: u32 = 1u32;
pub const SOCKET_SETTINGS_IPSEC_ALLOW_FIRST_INBOUND_PKT_UNENCRYPTED: u32 = 4u32;
pub const SOCKET_SETTINGS_IPSEC_OPTIONAL_PEER_NAME_VERIFICATION: u32 = 2u32;
pub const SOCKET_SETTINGS_IPSEC_PEER_NAME_IS_RAW_FORMAT: u32 = 8u32;
pub const SOCKET_SETTINGS_IPSEC_SKIP_FILTER_INSTANTIATION: u32 = 1u32;
#[repr(transparent)]
pub struct SOCKET_USAGE_TYPE(pub i32);
pub const SYSTEM_CRITICAL_SOCKET: SOCKET_USAGE_TYPE = SOCKET_USAGE_TYPE(1i32);
pub const SOCK_DGRAM: u16 = 2u16;
pub const SOCK_NOTIFY_EVENT_ERR: u32 = 64u32;
pub const SOCK_NOTIFY_EVENT_HANGUP: u32 = 4u32;
pub const SOCK_NOTIFY_EVENT_IN: u32 = 1u32;
pub const SOCK_NOTIFY_EVENT_OUT: u32 = 2u32;
pub const SOCK_NOTIFY_EVENT_REMOVE: u32 = 128u32;
pub const SOCK_NOTIFY_OP_DISABLE: u32 = 2u32;
pub const SOCK_NOTIFY_OP_ENABLE: u32 = 1u32;
pub const SOCK_NOTIFY_OP_NONE: u32 = 0u32;
pub const SOCK_NOTIFY_OP_REMOVE: u32 = 4u32;
pub const SOCK_NOTIFY_REGISTER_EVENT_HANGUP: u32 = 4u32;
pub const SOCK_NOTIFY_REGISTER_EVENT_IN: u32 = 1u32;
pub const SOCK_NOTIFY_REGISTER_EVENT_NONE: u32 = 0u32;
pub const SOCK_NOTIFY_REGISTER_EVENT_OUT: u32 = 2u32;
#[repr(C)]
pub struct SOCK_NOTIFY_REGISTRATION(i32);
pub const SOCK_NOTIFY_TRIGGER_EDGE: u32 = 8u32;
pub const SOCK_NOTIFY_TRIGGER_LEVEL: u32 = 4u32;
pub const SOCK_NOTIFY_TRIGGER_ONESHOT: u32 = 1u32;
pub const SOCK_NOTIFY_TRIGGER_PERSISTENT: u32 = 2u32;
pub const SOCK_RAW: u16 = 3u16;
pub const SOCK_RDM: u16 = 4u16;
pub const SOCK_SEQPACKET: u16 = 5u16;
pub const SOCK_STREAM: u16 = 1u16;
pub const SOL_IRLMP: u32 = 255u32;
pub const SOL_SOCKET: u32 = 65535u32;
pub const SOMAXCONN: u32 = 5u32;
pub const SO_ACCEPTCONN: u32 = 2u32;
pub const SO_BROADCAST: u32 = 32u32;
pub const SO_BSP_STATE: u32 = 4105u32;
pub const SO_COMPARTMENT_ID: u32 = 12292u32;
pub const SO_CONDITIONAL_ACCEPT: u32 = 12290u32;
pub const SO_CONNDATA: u32 = 28672u32;
pub const SO_CONNDATALEN: u32 = 28676u32;
pub const SO_CONNECT_TIME: u32 = 28684u32;
pub const SO_CONNOPT: u32 = 28673u32;
pub const SO_CONNOPTLEN: u32 = 28677u32;
pub const SO_DEBUG: u32 = 1u32;
pub const SO_DISCDATA: u32 = 28674u32;
pub const SO_DISCDATALEN: u32 = 28678u32;
pub const SO_DISCOPT: u32 = 28675u32;
pub const SO_DISCOPTLEN: u32 = 28679u32;
pub const SO_DONTROUTE: u32 = 16u32;
pub const SO_ERROR: u32 = 4103u32;
pub const SO_GROUP_ID: u32 = 8193u32;
pub const SO_GROUP_PRIORITY: u32 = 8194u32;
pub const SO_KEEPALIVE: u32 = 8u32;
pub const SO_LINGER: u32 = 128u32;
pub const SO_MAXDG: u32 = 28681u32;
pub const SO_MAXPATHDG: u32 = 28682u32;
pub const SO_MAX_MSG_SIZE: u32 = 8195u32;
pub const SO_OOBINLINE: u32 = 256u32;
pub const SO_OPENTYPE: u32 = 28680u32;
pub const SO_ORIGINAL_DST: u32 = 12303u32;
pub const SO_PAUSE_ACCEPT: u32 = 12291u32;
pub const SO_PORT_SCALABILITY: u32 = 12294u32;
pub const SO_PROTOCOL_INFO: u32 = 8197u32;
pub const SO_PROTOCOL_INFOA: u32 = 8196u32;
pub const SO_PROTOCOL_INFOW: u32 = 8197u32;
pub const SO_RANDOMIZE_PORT: u32 = 12293u32;
pub const SO_RCVBUF: u32 = 4098u32;
pub const SO_RCVLOWAT: u32 = 4100u32;
pub const SO_RCVTIMEO: u32 = 4102u32;
pub const SO_REUSEADDR: u32 = 4u32;
pub const SO_REUSE_MULTICASTPORT: u32 = 12296u32;
pub const SO_REUSE_UNICASTPORT: u32 = 12295u32;
pub const SO_SNDBUF: u32 = 4097u32;
pub const SO_SNDLOWAT: u32 = 4099u32;
pub const SO_SNDTIMEO: u32 = 4101u32;
pub const SO_SYNCHRONOUS_ALERT: u32 = 16u32;
pub const SO_SYNCHRONOUS_NONALERT: u32 = 32u32;
pub const SO_TIMESTAMP: u32 = 12298u32;
pub const SO_TIMESTAMP_ID: u32 = 12299u32;
pub const SO_TYPE: u32 = 4104u32;
pub const SO_UPDATE_ACCEPT_CONTEXT: u32 = 28683u32;
pub const SO_UPDATE_CONNECT_CONTEXT: u32 = 28688u32;
pub const SO_USELOOPBACK: u32 = 64u32;
#[repr(transparent)]
pub struct TCPSTATE(pub i32);
pub const TCPSTATE_CLOSED: TCPSTATE = TCPSTATE(0i32);
pub const TCPSTATE_LISTEN: TCPSTATE = TCPSTATE(1i32);
pub const TCPSTATE_SYN_SENT: TCPSTATE = TCPSTATE(2i32);
pub const TCPSTATE_SYN_RCVD: TCPSTATE = TCPSTATE(3i32);
pub const TCPSTATE_ESTABLISHED: TCPSTATE = TCPSTATE(4i32);
pub const TCPSTATE_FIN_WAIT_1: TCPSTATE = TCPSTATE(5i32);
pub const TCPSTATE_FIN_WAIT_2: TCPSTATE = TCPSTATE(6i32);
pub const TCPSTATE_CLOSE_WAIT: TCPSTATE = TCPSTATE(7i32);
pub const TCPSTATE_CLOSING: TCPSTATE = TCPSTATE(8i32);
pub const TCPSTATE_LAST_ACK: TCPSTATE = TCPSTATE(9i32);
pub const TCPSTATE_TIME_WAIT: TCPSTATE = TCPSTATE(10i32);
pub const TCPSTATE_MAX: TCPSTATE = TCPSTATE(11i32);
#[repr(C)]
pub struct TCP_ACK_FREQUENCY_PARAMETERS(i32);
pub const TCP_ATMARK: u32 = 8u32;
pub const TCP_BSDURGENT: u32 = 28672u32;
pub const TCP_CONGESTION_ALGORITHM: u32 = 12u32;
pub const TCP_DELAY_FIN_ACK: u32 = 13u32;
pub const TCP_EXPEDITED_1122: u32 = 2u32;
pub const TCP_FAIL_CONNECT_ON_ICMP_ERROR: u32 = 18u32;
pub const TCP_FASTOPEN: u32 = 15u32;
pub const TCP_ICMP_ERROR_INFO: u32 = 19u32;
#[repr(transparent)]
pub struct TCP_ICW_LEVEL(pub i32);
pub const TCP_ICW_LEVEL_DEFAULT: TCP_ICW_LEVEL = TCP_ICW_LEVEL(0i32);
pub const TCP_ICW_LEVEL_HIGH: TCP_ICW_LEVEL = TCP_ICW_LEVEL(1i32);
pub const TCP_ICW_LEVEL_VERY_HIGH: TCP_ICW_LEVEL = TCP_ICW_LEVEL(2i32);
pub const TCP_ICW_LEVEL_AGGRESSIVE: TCP_ICW_LEVEL = TCP_ICW_LEVEL(3i32);
pub const TCP_ICW_LEVEL_EXPERIMENTAL: TCP_ICW_LEVEL = TCP_ICW_LEVEL(4i32);
pub const TCP_ICW_LEVEL_COMPAT: TCP_ICW_LEVEL = TCP_ICW_LEVEL(254i32);
pub const TCP_ICW_LEVEL_MAX: TCP_ICW_LEVEL = TCP_ICW_LEVEL(255i32);
#[repr(C)]
pub struct TCP_ICW_PARAMETERS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TCP_INFO_v0(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TCP_INFO_v1(i32);
pub const TCP_INITIAL_RTO_DEFAULT_MAX_SYN_RETRANSMISSIONS: u32 = 0u32;
pub const TCP_INITIAL_RTO_DEFAULT_RTT: u32 = 0u32;
#[repr(C)]
pub struct TCP_INITIAL_RTO_PARAMETERS(i32);
pub const TCP_KEEPALIVE: u32 = 3u32;
pub const TCP_KEEPCNT: u32 = 16u32;
pub const TCP_KEEPIDLE: u32 = 3u32;
pub const TCP_KEEPINTVL: u32 = 17u32;
pub const TCP_MAXRT: u32 = 5u32;
pub const TCP_MAXRTMS: u32 = 14u32;
pub const TCP_MAXSEG: u32 = 4u32;
pub const TCP_NODELAY: u32 = 1u32;
pub const TCP_NOSYNRETRIES: u32 = 9u32;
pub const TCP_NOURG: u32 = 7u32;
pub const TCP_OFFLOAD_NOT_PREFERRED: u32 = 1u32;
pub const TCP_OFFLOAD_NO_PREFERENCE: u32 = 0u32;
pub const TCP_OFFLOAD_PREFERENCE: u32 = 11u32;
pub const TCP_OFFLOAD_PREFERRED: u32 = 2u32;
pub const TCP_STDURG: u32 = 6u32;
pub const TCP_TIMESTAMPS: u32 = 10u32;
pub const TF_DISCONNECT: u32 = 1u32;
pub const TF_REUSE_SOCKET: u32 = 2u32;
pub const TF_USE_DEFAULT_WORKER: u32 = 0u32;
pub const TF_USE_KERNEL_APC: u32 = 32u32;
pub const TF_USE_SYSTEM_THREAD: u32 = 16u32;
pub const TF_WRITE_BEHIND: u32 = 4u32;
pub const TH_NETDEV: u32 = 1u32;
pub const TH_TAPI: u32 = 2u32;
#[repr(C)]
pub struct TIMESTAMPING_CONFIG(i32);
pub const TIMESTAMPING_FLAG_RX: u32 = 1u32;
pub const TIMESTAMPING_FLAG_TX: u32 = 2u32;
pub const TNS_PLAN_CARRIER_ID_CODE: u32 = 1u32;
pub const TNS_TYPE_NATIONAL: u32 = 64u32;
pub const TP_DISCONNECT: u32 = 1u32;
pub const TP_ELEMENT_EOP: u32 = 4u32;
pub const TP_ELEMENT_FILE: u32 = 2u32;
pub const TP_ELEMENT_MEMORY: u32 = 1u32;
pub const TP_REUSE_SOCKET: u32 = 2u32;
pub const TP_USE_DEFAULT_WORKER: u32 = 0u32;
pub const TP_USE_KERNEL_APC: u32 = 32u32;
pub const TP_USE_SYSTEM_THREAD: u32 = 16u32;
#[repr(C)]
pub struct TRANSMIT_FILE_BUFFERS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct TRANSMIT_PACKETS_ELEMENT(i32);
#[repr(C)]
pub struct TRANSPORT_SETTING_ID(i32);
pub const TR_END_TO_END: u32 = 1u32;
pub const TR_NOIND: u32 = 0u32;
pub const TR_NO_END_TO_END: u32 = 2u32;
pub const TT_CBR: u32 = 4u32;
pub const TT_NOIND: u32 = 0u32;
pub const TT_VBR: u32 = 8u32;
pub const UDP_CHECKSUM_COVERAGE: u32 = 20u32;
pub const UDP_COALESCED_INFO: u32 = 3u32;
pub const UDP_NOCHECKSUM: u32 = 1u32;
pub const UDP_RECV_MAX_COALESCED_SIZE: u32 = 3u32;
pub const UDP_SEND_MSG_SIZE: u32 = 2u32;
pub const UNIX_PATH_MAX: u32 = 108u32;
pub const UP_P2MP: u32 = 1u32;
pub const UP_P2P: u32 = 0u32;
pub const VNSPROTO_IPC: u32 = 1u32;
pub const VNSPROTO_RELIABLE_IPC: u32 = 2u32;
pub const VNSPROTO_SPP: u32 = 3u32;
pub const WCE_AF_IRDA: u32 = 22u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WCE_DEVICELIST(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WCE_IRDA_DEVICE_INFO(i32);
pub const WCE_PF_IRDA: u32 = 22u32;
pub const WINDOWS_AF_IRDA: u32 = 26u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WINDOWS_DEVICELIST(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WINDOWS_IAS_QUERY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WINDOWS_IAS_SET(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WINDOWS_IRDA_DEVICE_INFO(i32);
pub const WINDOWS_PF_IRDA: u32 = 26u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSABUF(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[repr(C)]
pub struct WSACOMPLETION(i32);
#[repr(transparent)]
pub struct WSACOMPLETIONTYPE(pub i32);
pub const NSP_NOTIFY_IMMEDIATELY: WSACOMPLETIONTYPE = WSACOMPLETIONTYPE(0i32);
pub const NSP_NOTIFY_HWND: WSACOMPLETIONTYPE = WSACOMPLETIONTYPE(1i32);
pub const NSP_NOTIFY_EVENT: WSACOMPLETIONTYPE = WSACOMPLETIONTYPE(2i32);
pub const NSP_NOTIFY_PORT: WSACOMPLETIONTYPE = WSACOMPLETIONTYPE(3i32);
pub const NSP_NOTIFY_APC: WSACOMPLETIONTYPE = WSACOMPLETIONTYPE(4i32);
pub const WSADESCRIPTION_LEN: u32 = 256u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSAData(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSAData(i32);
#[repr(transparent)]
pub struct WSAECOMPARATOR(pub i32);
pub const COMP_EQUAL: WSAECOMPARATOR = WSAECOMPARATOR(0i32);
pub const COMP_NOTLESS: WSAECOMPARATOR = WSAECOMPARATOR(1i32);
#[repr(transparent)]
pub struct WSAESETSERVICEOP(pub i32);
pub const RNRSERVICE_REGISTER: WSAESETSERVICEOP = WSAESETSERVICEOP(0i32);
pub const RNRSERVICE_DEREGISTER: WSAESETSERVICEOP = WSAESETSERVICEOP(1i32);
pub const RNRSERVICE_DELETE: WSAESETSERVICEOP = WSAESETSERVICEOP(2i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSAMSG(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSANAMESPACE_INFOA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct WSANAMESPACE_INFOEXA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct WSANAMESPACE_INFOEXW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSANAMESPACE_INFOW(i32);
#[repr(C)]
pub struct WSANETWORKEVENTS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSANSCLASSINFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSANSCLASSINFOW(i32);
#[repr(C)]
pub struct WSAPOLLDATA(i32);
#[repr(C)]
pub struct WSAPOLLFD(i32);
#[repr(C)]
pub struct WSAPROTOCOLCHAIN(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSAPROTOCOL_INFOA(i32);
#[repr(C)]
pub struct WSAPROTOCOL_INFOW(i32);
pub const WSAPROTOCOL_LEN: u32 = 255u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct WSAQUERYSET2A(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct WSAQUERYSET2W(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct WSAQUERYSETA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct WSAQUERYSETW(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[repr(C)]
pub struct WSASENDMSG(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSASERVICECLASSINFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSASERVICECLASSINFOW(i32);
pub const WSASYS_STATUS_LEN: u32 = 128u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSATHREADID(i32);
#[repr(C)]
pub struct WSAVERSION(i32);
#[repr(transparent)]
pub struct WSA_COMPATIBILITY_BEHAVIOR_ID(pub i32);
pub const WsaBehaviorAll: WSA_COMPATIBILITY_BEHAVIOR_ID = WSA_COMPATIBILITY_BEHAVIOR_ID(0i32);
pub const WsaBehaviorReceiveBuffering: WSA_COMPATIBILITY_BEHAVIOR_ID = WSA_COMPATIBILITY_BEHAVIOR_ID(1i32);
pub const WsaBehaviorAutoTuning: WSA_COMPATIBILITY_BEHAVIOR_ID = WSA_COMPATIBILITY_BEHAVIOR_ID(2i32);
#[repr(C)]
pub struct WSA_COMPATIBILITY_MODE(i32);
#[repr(transparent)]
pub struct WSA_ERROR(pub i32);
pub const WSA_IO_PENDING: WSA_ERROR = WSA_ERROR(997i32);
pub const WSA_IO_INCOMPLETE: WSA_ERROR = WSA_ERROR(996i32);
pub const WSA_INVALID_HANDLE: WSA_ERROR = WSA_ERROR(6i32);
pub const WSA_INVALID_PARAMETER: WSA_ERROR = WSA_ERROR(87i32);
pub const WSA_NOT_ENOUGH_MEMORY: WSA_ERROR = WSA_ERROR(8i32);
pub const WSA_OPERATION_ABORTED: WSA_ERROR = WSA_ERROR(995i32);
pub const WSABASEERR: WSA_ERROR = WSA_ERROR(10000i32);
pub const WSAEINTR: WSA_ERROR = WSA_ERROR(10004i32);
pub const WSAEBADF: WSA_ERROR = WSA_ERROR(10009i32);
pub const WSAEACCES: WSA_ERROR = WSA_ERROR(10013i32);
pub const WSAEFAULT: WSA_ERROR = WSA_ERROR(10014i32);
pub const WSAEINVAL: WSA_ERROR = WSA_ERROR(10022i32);
pub const WSAEMFILE: WSA_ERROR = WSA_ERROR(10024i32);
pub const WSAEWOULDBLOCK: WSA_ERROR = WSA_ERROR(10035i32);
pub const WSAEINPROGRESS: WSA_ERROR = WSA_ERROR(10036i32);
pub const WSAEALREADY: WSA_ERROR = WSA_ERROR(10037i32);
pub const WSAENOTSOCK: WSA_ERROR = WSA_ERROR(10038i32);
pub const WSAEDESTADDRREQ: WSA_ERROR = WSA_ERROR(10039i32);
pub const WSAEMSGSIZE: WSA_ERROR = WSA_ERROR(10040i32);
pub const WSAEPROTOTYPE: WSA_ERROR = WSA_ERROR(10041i32);
pub const WSAENOPROTOOPT: WSA_ERROR = WSA_ERROR(10042i32);
pub const WSAEPROTONOSUPPORT: WSA_ERROR = WSA_ERROR(10043i32);
pub const WSAESOCKTNOSUPPORT: WSA_ERROR = WSA_ERROR(10044i32);
pub const WSAEOPNOTSUPP: WSA_ERROR = WSA_ERROR(10045i32);
pub const WSAEPFNOSUPPORT: WSA_ERROR = WSA_ERROR(10046i32);
pub const WSAEAFNOSUPPORT: WSA_ERROR = WSA_ERROR(10047i32);
pub const WSAEADDRINUSE: WSA_ERROR = WSA_ERROR(10048i32);
pub const WSAEADDRNOTAVAIL: WSA_ERROR = WSA_ERROR(10049i32);
pub const WSAENETDOWN: WSA_ERROR = WSA_ERROR(10050i32);
pub const WSAENETUNREACH: WSA_ERROR = WSA_ERROR(10051i32);
pub const WSAENETRESET: WSA_ERROR = WSA_ERROR(10052i32);
pub const WSAECONNABORTED: WSA_ERROR = WSA_ERROR(10053i32);
pub const WSAECONNRESET: WSA_ERROR = WSA_ERROR(10054i32);
pub const WSAENOBUFS: WSA_ERROR = WSA_ERROR(10055i32);
pub const WSAEISCONN: WSA_ERROR = WSA_ERROR(10056i32);
pub const WSAENOTCONN: WSA_ERROR = WSA_ERROR(10057i32);
pub const WSAESHUTDOWN: WSA_ERROR = WSA_ERROR(10058i32);
pub const WSAETOOMANYREFS: WSA_ERROR = WSA_ERROR(10059i32);
pub const WSAETIMEDOUT: WSA_ERROR = WSA_ERROR(10060i32);
pub const WSAECONNREFUSED: WSA_ERROR = WSA_ERROR(10061i32);
pub const WSAELOOP: WSA_ERROR = WSA_ERROR(10062i32);
pub const WSAENAMETOOLONG: WSA_ERROR = WSA_ERROR(10063i32);
pub const WSAEHOSTDOWN: WSA_ERROR = WSA_ERROR(10064i32);
pub const WSAEHOSTUNREACH: WSA_ERROR = WSA_ERROR(10065i32);
pub const WSAENOTEMPTY: WSA_ERROR = WSA_ERROR(10066i32);
pub const WSAEPROCLIM: WSA_ERROR = WSA_ERROR(10067i32);
pub const WSAEUSERS: WSA_ERROR = WSA_ERROR(10068i32);
pub const WSAEDQUOT: WSA_ERROR = WSA_ERROR(10069i32);
pub const WSAESTALE: WSA_ERROR = WSA_ERROR(10070i32);
pub const WSAEREMOTE: WSA_ERROR = WSA_ERROR(10071i32);
pub const WSASYSNOTREADY: WSA_ERROR = WSA_ERROR(10091i32);
pub const WSAVERNOTSUPPORTED: WSA_ERROR = WSA_ERROR(10092i32);
pub const WSANOTINITIALISED: WSA_ERROR = WSA_ERROR(10093i32);
pub const WSAEDISCON: WSA_ERROR = WSA_ERROR(10101i32);
pub const WSAENOMORE: WSA_ERROR = WSA_ERROR(10102i32);
pub const WSAECANCELLED: WSA_ERROR = WSA_ERROR(10103i32);
pub const WSAEINVALIDPROCTABLE: WSA_ERROR = WSA_ERROR(10104i32);
pub const WSAEINVALIDPROVIDER: WSA_ERROR = WSA_ERROR(10105i32);
pub const WSAEPROVIDERFAILEDINIT: WSA_ERROR = WSA_ERROR(10106i32);
pub const WSASYSCALLFAILURE: WSA_ERROR = WSA_ERROR(10107i32);
pub const WSASERVICE_NOT_FOUND: WSA_ERROR = WSA_ERROR(10108i32);
pub const WSATYPE_NOT_FOUND: WSA_ERROR = WSA_ERROR(10109i32);
pub const WSA_E_NO_MORE: WSA_ERROR = WSA_ERROR(10110i32);
pub const WSA_E_CANCELLED: WSA_ERROR = WSA_ERROR(10111i32);
pub const WSAEREFUSED: WSA_ERROR = WSA_ERROR(10112i32);
pub const WSAHOST_NOT_FOUND: WSA_ERROR = WSA_ERROR(11001i32);
pub const WSATRY_AGAIN: WSA_ERROR = WSA_ERROR(11002i32);
pub const WSANO_RECOVERY: WSA_ERROR = WSA_ERROR(11003i32);
pub const WSANO_DATA: WSA_ERROR = WSA_ERROR(11004i32);
pub const WSA_QOS_RECEIVERS: WSA_ERROR = WSA_ERROR(11005i32);
pub const WSA_QOS_SENDERS: WSA_ERROR = WSA_ERROR(11006i32);
pub const WSA_QOS_NO_SENDERS: WSA_ERROR = WSA_ERROR(11007i32);
pub const WSA_QOS_NO_RECEIVERS: WSA_ERROR = WSA_ERROR(11008i32);
pub const WSA_QOS_REQUEST_CONFIRMED: WSA_ERROR = WSA_ERROR(11009i32);
pub const WSA_QOS_ADMISSION_FAILURE: WSA_ERROR = WSA_ERROR(11010i32);
pub const WSA_QOS_POLICY_FAILURE: WSA_ERROR = WSA_ERROR(11011i32);
pub const WSA_QOS_BAD_STYLE: WSA_ERROR = WSA_ERROR(11012i32);
pub const WSA_QOS_BAD_OBJECT: WSA_ERROR = WSA_ERROR(11013i32);
pub const WSA_QOS_TRAFFIC_CTRL_ERROR: WSA_ERROR = WSA_ERROR(11014i32);
pub const WSA_QOS_GENERIC_ERROR: WSA_ERROR = WSA_ERROR(11015i32);
pub const WSA_QOS_ESERVICETYPE: WSA_ERROR = WSA_ERROR(11016i32);
pub const WSA_QOS_EFLOWSPEC: WSA_ERROR = WSA_ERROR(11017i32);
pub const WSA_QOS_EPROVSPECBUF: WSA_ERROR = WSA_ERROR(11018i32);
pub const WSA_QOS_EFILTERSTYLE: WSA_ERROR = WSA_ERROR(11019i32);
pub const WSA_QOS_EFILTERTYPE: WSA_ERROR = WSA_ERROR(11020i32);
pub const WSA_QOS_EFILTERCOUNT: WSA_ERROR = WSA_ERROR(11021i32);
pub const WSA_QOS_EOBJLENGTH: WSA_ERROR = WSA_ERROR(11022i32);
pub const WSA_QOS_EFLOWCOUNT: WSA_ERROR = WSA_ERROR(11023i32);
pub const WSA_QOS_EUNKOWNPSOBJ: WSA_ERROR = WSA_ERROR(11024i32);
pub const WSA_QOS_EPOLICYOBJ: WSA_ERROR = WSA_ERROR(11025i32);
pub const WSA_QOS_EFLOWDESC: WSA_ERROR = WSA_ERROR(11026i32);
pub const WSA_QOS_EPSFLOWSPEC: WSA_ERROR = WSA_ERROR(11027i32);
pub const WSA_QOS_EPSFILTERSPEC: WSA_ERROR = WSA_ERROR(11028i32);
pub const WSA_QOS_ESDMODEOBJ: WSA_ERROR = WSA_ERROR(11029i32);
pub const WSA_QOS_ESHAPERATEOBJ: WSA_ERROR = WSA_ERROR(11030i32);
pub const WSA_QOS_RESERVED_PETYPE: WSA_ERROR = WSA_ERROR(11031i32);
pub const WSA_SECURE_HOST_NOT_FOUND: WSA_ERROR = WSA_ERROR(11032i32);
pub const WSA_IPSEC_NAME_POLICY_ERROR: WSA_ERROR = WSA_ERROR(11033i32);
pub const WSA_FLAG_ACCESS_SYSTEM_SECURITY: u32 = 64u32;
pub const WSA_FLAG_MULTIPOINT_C_LEAF: u32 = 4u32;
pub const WSA_FLAG_MULTIPOINT_C_ROOT: u32 = 2u32;
pub const WSA_FLAG_MULTIPOINT_D_LEAF: u32 = 16u32;
pub const WSA_FLAG_MULTIPOINT_D_ROOT: u32 = 8u32;
pub const WSA_FLAG_NO_HANDLE_INHERIT: u32 = 128u32;
pub const WSA_FLAG_OVERLAPPED: u32 = 1u32;
pub const WSA_FLAG_REGISTERED_IO: u32 = 256u32;
pub const WSA_INFINITE: u32 = 4294967295u32;
pub const WSA_MAXIMUM_WAIT_EVENTS: u32 = 64u32;
pub const WSA_WAIT_EVENT_0: u32 = 0u32;
pub const WSA_WAIT_FAILED: u32 = 4294967295u32;
pub const WSA_WAIT_IO_COMPLETION: u32 = 192u32;
#[repr(C)]
pub struct WSC_PROVIDER_AUDIT_INFO(i32);
#[repr(transparent)]
pub struct WSC_PROVIDER_INFO_TYPE(pub i32);
pub const ProviderInfoLspCategories: WSC_PROVIDER_INFO_TYPE = WSC_PROVIDER_INFO_TYPE(0i32);
pub const ProviderInfoAudit: WSC_PROVIDER_INFO_TYPE = WSC_PROVIDER_INFO_TYPE(1i32);
pub const WSK_SO_BASE: u32 = 16384u32;
pub const WSPDESCRIPTION_LEN: u32 = 255u32;
#[repr(C)]
pub struct WSPData(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_QoS", feature = "Win32_System_IO"))]
#[repr(C)]
pub struct WSPPROC_TABLE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WSPUPCALLTABLE(i32);
pub const WSS_OPERATION_IN_PROGRESS: i32 = 259i32;
pub const XP1_CONNECTIONLESS: u32 = 1u32;
pub const XP1_CONNECT_DATA: u32 = 128u32;
pub const XP1_DISCONNECT_DATA: u32 = 256u32;
pub const XP1_EXPEDITED_DATA: u32 = 64u32;
pub const XP1_GRACEFUL_CLOSE: u32 = 32u32;
pub const XP1_GUARANTEED_DELIVERY: u32 = 2u32;
pub const XP1_GUARANTEED_ORDER: u32 = 4u32;
pub const XP1_IFS_HANDLES: u32 = 131072u32;
pub const XP1_INTERRUPT: u32 = 16384u32;
pub const XP1_MESSAGE_ORIENTED: u32 = 8u32;
pub const XP1_MULTIPOINT_CONTROL_PLANE: u32 = 2048u32;
pub const XP1_MULTIPOINT_DATA_PLANE: u32 = 4096u32;
pub const XP1_PARTIAL_MESSAGE: u32 = 262144u32;
pub const XP1_PSEUDO_STREAM: u32 = 16u32;
pub const XP1_QOS_SUPPORTED: u32 = 8192u32;
pub const XP1_SAN_SUPPORT_SDP: u32 = 524288u32;
pub const XP1_SUPPORT_BROADCAST: u32 = 512u32;
pub const XP1_SUPPORT_MULTIPOINT: u32 = 1024u32;
pub const XP1_UNI_RECV: u32 = 65536u32;
pub const XP1_UNI_SEND: u32 = 32768u32;
pub const XP_BANDWIDTH_ALLOCATION: u32 = 2048u32;
pub const XP_CONNECTIONLESS: u32 = 1u32;
pub const XP_CONNECT_DATA: u32 = 128u32;
pub const XP_DISCONNECT_DATA: u32 = 256u32;
pub const XP_ENCRYPTS: u32 = 8192u32;
pub const XP_EXPEDITED_DATA: u32 = 64u32;
pub const XP_FRAGMENTATION: u32 = 4096u32;
pub const XP_GRACEFUL_CLOSE: u32 = 32u32;
pub const XP_GUARANTEED_DELIVERY: u32 = 2u32;
pub const XP_GUARANTEED_ORDER: u32 = 4u32;
pub const XP_MESSAGE_ORIENTED: u32 = 8u32;
pub const XP_PSEUDO_STREAM: u32 = 16u32;
pub const XP_SUPPORTS_BROADCAST: u32 = 512u32;
pub const XP_SUPPORTS_MULTICAST: u32 = 1024u32;
pub const _SS_MAXSIZE: u32 = 128u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct addrinfoW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct addrinfo_dns_server(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct addrinfoex2A(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct addrinfoex2W(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct addrinfoex3(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct addrinfoex4(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct addrinfoex5(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct addrinfoex6(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct addrinfoexA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct addrinfoexW(i32);
#[repr(C)]
pub struct cmsghdr(i32);
#[repr(transparent)]
pub struct eWINDOW_ADVANCE_METHOD(pub i32);
pub const E_WINDOW_ADVANCE_BY_TIME: eWINDOW_ADVANCE_METHOD = eWINDOW_ADVANCE_METHOD(1i32);
pub const E_WINDOW_USE_AS_DATA_CACHE: eWINDOW_ADVANCE_METHOD = eWINDOW_ADVANCE_METHOD(2i32);
#[repr(C)]
pub struct fd_set(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct hostent(i32);
#[repr(C)]
pub struct in6_pktinfo_ex(i32);
#[repr(C)]
pub struct linger(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct netent(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct protoent(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct servent(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct servent(i32);
#[repr(C)]
pub struct sockaddr_atm(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct sockaddr_gen(i32);
#[repr(C)]
pub struct sockaddr_in6_old(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct sockaddr_ipx(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct sockaddr_nb(i32);
#[repr(C)]
pub struct sockaddr_tp(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct sockaddr_un(i32);
#[repr(C)]
pub struct sockaddr_vns(i32);
#[repr(C)]
pub struct sockproto(i32);
#[repr(C)]
pub struct tcp_keepalive(i32);
#[repr(C)]
pub struct timeval(i32);