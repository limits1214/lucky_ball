//
//  Use this file to import your target's public headers that you would like to expose to Swift.
//
#import <Foundation/Foundation.h>
void ffi_callback_admob_interstitial_load_success();
void ffi_callback_admob_interstitial_load_fail(const char* errmsg);
void ffi_callback_admob_interstitial_showed();
void ffi_callback_admob_interstitial_show_fail(const char* errmsg);
void ffi_callback_admob_interstitial_dismissed();
void ffi_callback_admob_interstitial_is_ready(bool is_ready);
void ffi_callback_app_init_end();
