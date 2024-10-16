//
//  lucky_ball.swift
//  lucky ball
//
//  Created by SY L on 10/9/24.
//

import Foundation
import AppTrackingTransparency
@preconcurrency import GoogleMobileAds

@MainActor
class AdmobInterstitial: NSObject, @preconcurrency GADFullScreenContentDelegate {
    private var interstitial: GADInterstitialAd?
    
    func load()  {
        Task {
            do {
                #if DEBUG
                    let adUnitID = "ca-app-pub-3940256099942544/4411468910"
                #else
                    let adUnitID = "ca-app-pub-9994223855742968/9862909196"
                #endif
                
               interstitial = try await GADInterstitialAd.load(
                withAdUnitID: adUnitID, request: GADRequest())
                interstitial?.fullScreenContentDelegate = self;
                ffi_callback_admob_interstitial_load_success();
            } catch {
                let errMsg = error.localizedDescription;
                print("Failed to load interstitial ad with error: \(errMsg)")
                ffi_callback_admob_interstitial_load_fail(errMsg);
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
    
    func isReady() -> Bool {
        return interstitial != nil
    }
    
    func clear() {
        interstitial = nil
    }
    
    /// Tells the delegate that the ad failed to present full screen content.
     func ad(_ ad: GADFullScreenPresentingAd, didFailToPresentFullScreenContentWithError error: Error) {
         let errMsg = error.localizedDescription;
         print("Ad did fail to present full screen content. err: \(errMsg)")
         ffi_callback_admob_interstitial_show_fail(errMsg);
         interstitial = nil
     }

     /// Tells the delegate that the ad will present full screen content.
     func adWillPresentFullScreenContent(_ ad: GADFullScreenPresentingAd) {
         print("Ad will present full screen content.")
         ffi_callback_admob_interstitial_showed()
     }

     /// Tells the delegate that the ad dismissed full screen content.
     func adDidDismissFullScreenContent(_ ad: GADFullScreenPresentingAd) {
         print("Ad did dismiss full screen content.")
         ffi_callback_admob_interstitial_dismissed()
         interstitial = nil
     }
}
@MainActor
class AdmobBanner: NSObject, @preconcurrency GADBannerViewDelegate {
    var bannerView: GADBannerView!
    func launch(vc: UIViewController) {
        
        let viewWidth = vc.view.frame.inset(by: vc.view.safeAreaInsets).width
        let adaptiveSize = GADCurrentOrientationAnchoredAdaptiveBannerAdSizeWithWidth(viewWidth)
        bannerView = GADBannerView(adSize: adaptiveSize)
        
        addBannerViewToView(vc: vc, bannerView: bannerView)
        
        bannerView.delegate = self
        #if DEBUG
            let adUnitID = "ca-app-pub-3940256099942544/2435281174"
        #else
            let adUnitID = "ca-app-pub-9994223855742968/6496672969"
        #endif
        bannerView.adUnitID = adUnitID
        bannerView.rootViewController = vc
        bannerView.load(GADRequest())
    }
    
    func addBannerViewToView(vc: UIViewController, bannerView: GADBannerView) {
        bannerView.translatesAutoresizingMaskIntoConstraints = false
        vc.view.addSubview(bannerView)
        vc.view.addConstraints(
          [NSLayoutConstraint(item: bannerView,
                              attribute: .bottom,
                              relatedBy: .equal,
                              toItem: vc.view.safeAreaLayoutGuide,
                              attribute: .bottom,
                              multiplier: 1,
                              constant: 0),
           NSLayoutConstraint(item: bannerView,
                              attribute: .centerX,
                              relatedBy: .equal,
                              toItem: vc.view,
                              attribute: .centerX,
                              multiplier: 1,
                              constant: 0)
          ])
       }
    
    func bannerViewDidReceiveAd(_ bannerView: GADBannerView) {
      print("bannerViewDidReceiveAd")
    }

    func bannerView(_ bannerView: GADBannerView, didFailToReceiveAdWithError error: Error) {
      print("bannerView:didFailToReceiveAdWithError: \(error.localizedDescription)")
    }

    func bannerViewDidRecordImpression(_ bannerView: GADBannerView) {
      print("bannerViewDidRecordImpression")
    }

    func bannerViewWillPresentScreen(_ bannerView: GADBannerView) {
      print("bannerViewWillPresentScreen")
    }

    func bannerViewWillDismissScreen(_ bannerView: GADBannerView) {
      print("bannerViewWillDIsmissScreen")
    }

    func bannerViewDidDismissScreen(_ bannerView: GADBannerView) {
      print("bannerViewDidDismissScreen")
    }
}

@MainActor let admobInterstitial = AdmobInterstitial();

@_cdecl("ffi_admob_interstitial_show")
func ffi_admob_interstitial_show() {
    DispatchQueue.main.async {
        admobInterstitial.show();
    }
}

@_cdecl("ffi_admob_interstitial_load")
func ffi_admob_interstitial_load() {
    DispatchQueue.main.async {
        admobInterstitial.load();
    }
}

@_cdecl("ffi_admob_interstitial_is_ready")
func ffi_admob_interstitial_is_ready(){
    DispatchQueue.main.async {
        let isReady = admobInterstitial.isReady()
        print("ffi_admob_interstitial_is_ready: \(isReady)")
        ffi_callback_admob_interstitial_is_ready(isReady)
    }
}

@_cdecl("ffi_admob_interstitial_clear")
func ffi_admob_interstitial_clear() {
    DispatchQueue.main.async {
        admobInterstitial.clear();
    }
}

@MainActor let admobBanner = AdmobBanner();

@_cdecl("ffi_admob_banner_launch")
func ffi_admob_banner_launch(vc: UIViewController) {
    DispatchQueue.main.async {
        admobBanner.launch(vc: vc);
    }
}

///

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

///

@_cdecl("ffi_app_init")
public func ffi_app_init() {
    if #available(iOS 14, *) {
        ATTrackingManager.requestTrackingAuthorization(completionHandler: {status in
            GADMobileAds.sharedInstance().start() { _ in
                ffi_callback_app_init_end();
            }
        })
    } else {
        // Fallback on earlier versions
        GADMobileAds.sharedInstance().start() { _ in
            ffi_callback_app_init_end();
        }
    }
}

@_cdecl("ffi_app_exit")
public func ffi_app_exit() {
    print("ffi_app_exit");
}

@_cdecl("ffi_get_current_epoch_time")
public func ffi_get_current_epoch_time() -> Int {
    let currentEpochTime = Date().timeIntervalSince1970;
    return Int(currentEpochTime);
}

@_cdecl("ffi_get_locale")
public func ffi_get_current_epoch_time() -> UnsafePointer<CChar>? {
    let currentLocale = Locale.current
    let localeIdentifier = currentLocale.identifier
    print("\(localeIdentifier)")
    return (localeIdentifier as NSString).utf8String
}

@_cdecl("ffi_get_time_offset")
public func ffi_get_time_offset() ->  Int {
    let timeZone = TimeZone.current
    let secondsFromGMT = timeZone.secondsFromGMT()
//    let hoursFromGMT = secondsFromGMT / 3600
    return secondsFromGMT
}

//@_cdecl("ffi_request_idfa")
//public func ffi_request_att()  {
//    
//}
