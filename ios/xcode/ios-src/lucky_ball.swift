//
//  lucky_ball.swift
//  lucky ball
//
//  Created by SY L on 10/9/24.
//

import Foundation
@preconcurrency import GoogleMobileAds
@MainActor
class AdmobInterstitial: NSObject, @preconcurrency GADFullScreenContentDelegate {
    private var interstitial: GADInterstitialAd?
    
    func load()  {
        Task {
            do {
               interstitial = try await GADInterstitialAd.load(
                withAdUnitID: "ca-app-pub-3940256099942544/4411468910", request: GADRequest())
                interstitial?.fullScreenContentDelegate = self;
            } catch {
              print("Failed to load interstitial ad with error: \(error.localizedDescription)")
            }
        }
    }
    func show() {
        print("show");
        guard let interstitial = interstitial else {
          return print("Ad wasn't ready.")
        }

        // The UIViewController parameter is an optional.
        interstitial.present(fromRootViewController: nil)
    }
    
    /// Tells the delegate that the ad failed to present full screen content.
     func ad(_ ad: GADFullScreenPresentingAd, didFailToPresentFullScreenContentWithError error: Error) {
       print("Ad did fail to present full screen content.")
     }

     /// Tells the delegate that the ad will present full screen content.
     func adWillPresentFullScreenContent(_ ad: GADFullScreenPresentingAd) {
       print("Ad will present full screen content.")
     }

     /// Tells the delegate that the ad dismissed full screen content.
     func adDidDismissFullScreenContent(_ ad: GADFullScreenPresentingAd) {
       print("Ad did dismiss full screen content.")
         
     }
}

@MainActor let admobInterstitial = AdmobInterstitial();

@_cdecl("ffi_ad_init")
public func ffi_ad_init() {
    GADMobileAds.sharedInstance().start() { _ in
        print("init end");
    }
}

@MainActor @_cdecl("ffi_admob_interstial_show")
func ffi_admob_interstial_show() {
    admobInterstitial.show();
}

@MainActor @_cdecl("ffi_admob_interstial_load")
func ffi_admob_interstial_load() {
    admobInterstitial.load();
}

@_cdecl("ffi_kv_get")
func ffi_kv_get(key: UnsafePointer<CChar>) -> UnsafePointer<CChar>?{
    let keyString = String(cString: key)
    let value = UserDefaults.standard.string(forKey: keyString) ?? ""
    return (value as NSString).utf8String
}

@_cdecl("ffi_kv_set")
func ffi_kv_set(key: UnsafePointer<CChar>, value: UnsafePointer<CChar>) {
    let keyString = String(cString: key)
    let valString = String(cString: value)
    UserDefaults.standard.set(valString, forKey: keyString);
}

@_cdecl("ffi_kv_delete")
func ffi_kv_delete(key: UnsafePointer<CChar>) {
    let keyString = String(cString: key)
    UserDefaults.standard.removeObject(forKey: keyString)
}

@_cdecl("ffi_kv_exists")
func ffi_kv_exists(key: UnsafePointer<CChar>) -> Bool {
    let keyString = String(cString: key)
    if UserDefaults.standard.object(forKey: keyString) != nil {
        return true
    } else {
        return false
    }
}
