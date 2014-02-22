/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/. */

/* automatically generated by rust-bindgen */

use libc;
use libc::*;
use jsapi::*;
use jsfriendapi::JSJitInfo;
use jsval::JSVal;

type c_bool = libc::c_int;

pub struct ProxyTraps {
    pub getPropertyDescriptor: Option<extern "C" fn(*JSContext, *JSObject, jsid, c_bool, *mut JSPropertyDescriptor) -> c_bool>,
    pub getOwnPropertyDescriptor: Option<extern "C" fn(*JSContext, *JSObject, jsid, JSBool, *mut JSPropertyDescriptor) -> JSBool>,
    pub defineProperty: Option<extern "C" fn(*JSContext, *JSObject, jsid, *JSPropertyDescriptor) -> JSBool>,
    pub getOwnPropertyNames: *u8, //XXX need a representation for AutoIdVector&
    pub delete_: Option<extern "C" fn(*JSContext, *JSObject, jsid, *bool) -> JSBool>,
    pub enumerate: *u8, //XXX need a representation for AutoIdVector&

    pub has: Option<extern "C" fn(*JSContext, *JSObject, jsid, *mut JSBool) -> JSBool>,
    pub hasOwn: Option<extern "C" fn(*JSContext, *JSObject, jsid, *mut JSBool) -> JSBool>,
    pub get: Option<extern "C" fn(*JSContext, *JSObject, *JSObject, jsid, *mut JSVal) -> JSBool>,
    pub set: Option<extern "C" fn(*JSContext, *JSObject, *JSObject, jsid, JSBool, *JSVal) -> JSBool>,
    pub keys: *u8, //XXX need a representation for AutoIdVector&
    pub iterate: Option<extern "C" fn(*JSContext, *JSObject, uint, *JSVal) -> JSBool>,

    pub call: Option<extern "C" fn(*JSContext, *JSObject, uint, *JSVal) -> JSBool>,
    pub construct: Option<extern "C" fn(*JSContext, *JSObject, uint, *JSVal, *JSVal) -> JSBool>,
    pub nativeCall: *u8, //XXX need a representation for IsAcceptableThis, NativeImpl, and CallArgs
    pub hasInstance: Option<extern "C" fn(*JSContext, *JSObject, *JSVal, *JSBool) -> JSBool>,
    pub typeOf: Option<extern "C" fn(*JSContext, *JSObject) -> uint>, //XXX JSType enum
    pub objectClassIs: Option<extern "C" fn(*JSObject, uint, *JSContext) -> JSBool>, //XXX ESClassValue enum
    pub obj_toString: Option<extern "C" fn(*JSContext, *JSObject) -> *JSString>,
    pub fun_toString: Option<extern "C" fn(*JSContext, *JSObject, uint) -> *JSString>,
    //regexp_toShared: *u8,
    pub defaultValue: Option<extern "C" fn(*JSContext, *JSObject, uint, *JSVal) -> JSBool>, //XXX JSType enum
    pub iteratorNext: Option<extern "C" fn(*JSContext, *JSObject, *JSVal) -> JSBool>,
    pub finalize: Option<extern "C" fn(*JSFreeOp, *JSObject)>,
    pub getElementIfPresent: Option<extern "C" fn(*JSContext, *JSObject, *JSObject, u32, *JSVal, *JSBool) -> JSBool>,
    pub getPrototypeOf: Option<extern "C" fn(*JSContext, *JSObject, **JSObject) -> JSBool>,
    pub trace: Option<extern "C" fn(*mut JSTracer, *JSObject)>,
}

#[cfg(not(target_os = "android"))]
#[link(name = "jsglue")]
extern { }


#[cfg(target_os = "android")]
#[link_args = "-ljsglue -lstdc++ -lgcc"]
extern { }

extern {

//#[rust_stack]
pub fn RUST_JS_NumberValue(d: f64) -> JSVal;

//#[rust_stack]
pub fn CallJitPropertyOp(info: *JSJitInfo, cx: *JSContext, thisObj: *JSObject, specializedThis: *libc::c_void, vp: *JSVal) -> JSBool;

//#[rust_stack]
pub fn CallJitMethodOp(info: *JSJitInfo, cx: *JSContext, thisObj: *JSObject, specializedThis: *libc::c_void, argc: libc::c_uint, vp: *JSVal) -> JSBool;

//#[rust_stack]
pub fn RUST_FUNCTION_VALUE_TO_JITINFO(v: JSVal) -> *JSJitInfo;

pub fn SetFunctionNativeReserved(fun: *JSObject, which: libc::size_t, val: *JSVal);
pub fn GetFunctionNativeReserved(fun: *JSObject, which: libc::size_t) -> *JSVal;

pub fn CreateProxyHandler(traps: *ProxyTraps, extra: *libc::c_void) -> *libc::c_void;
pub fn CreateWrapperProxyHandler(traps: *ProxyTraps) -> *libc::c_void;
pub fn NewProxyObject(cx: *JSContext, handler: *libc::c_void, priv_: *JSVal,
                      proto: *JSObject, parent: *JSObject, call: *JSObject,
                      construct: *JSObject) -> *JSObject;
pub fn WrapperNew(cx: *JSContext, parent: *JSObject, handler: *libc::c_void) -> *JSObject;

pub fn GetProxyExtra(obj: *JSObject, slot: c_uint) -> JSVal;
pub fn GetProxyPrivate(obj: *JSObject) -> JSVal;
pub fn SetProxyExtra(obj: *JSObject, slot: c_uint, val: JSVal);

pub fn GetObjectProto(obj: *JSObject) -> *JSObject;
pub fn GetObjectParent(obj: *JSObject) -> *JSObject;

pub fn RUST_JSID_IS_INT(id: jsid) -> JSBool;
pub fn RUST_JSID_TO_INT(id: jsid) -> libc::c_int;
pub fn RUST_JSID_IS_STRING(id: jsid) -> JSBool;
pub fn RUST_JSID_TO_STRING(id: jsid) -> *JSString;

pub fn RUST_SET_JITINFO(func: *JSFunction, info: *JSJitInfo);

pub fn RUST_INTERNED_STRING_TO_JSID(cx: *JSContext, str: *JSString) -> jsid;

pub fn DefineFunctionWithReserved(cx: *JSContext, obj: *JSObject,
                                  name: *libc::c_char, call: JSNative, nargs: libc::c_uint,
                                  attrs: libc::c_uint) -> *JSObject;
pub fn GetObjectJSClass(obj: *JSObject) -> *JSClass;
pub fn js_GetErrorMessage(userRef: *libc::c_void, locale: *libc::c_char,
                          errorNumber: libc::c_uint) -> *JSErrorFormatString;
pub fn js_IsObjectProxyClass(obj: *JSObject) -> bool;
pub fn js_IsFunctionProxyClass(obj: *JSObject) -> bool;
pub fn IsProxyHandlerFamily(obj: *JSObject) -> bool;
pub fn GetProxyHandlerExtra(obj: *JSObject) -> *libc::c_void;
pub fn GetProxyHandler(obj: *JSObject) -> *libc::c_void;
pub fn InvokeGetOwnPropertyDescriptor(handler: *libc::c_void, cx: *JSContext, proxy: *JSObject, id: jsid, set: JSBool, desc: *mut JSPropertyDescriptor) -> JSBool;
pub fn GetGlobalForObjectCrossCompartment(obj: *JSObject) -> *JSObject;
pub fn ReportError(cx: *JSContext, error: *c_char);
pub fn IsWrapper(obj: *JSObject) -> JSBool;
pub fn UnwrapObject(obj: *JSObject, stopAtOuter: JSBool, flags: *libc::c_uint) -> *JSObject;
}
