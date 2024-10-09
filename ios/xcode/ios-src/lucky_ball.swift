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


func aa() {
//    admobInterstitial.load();
}
