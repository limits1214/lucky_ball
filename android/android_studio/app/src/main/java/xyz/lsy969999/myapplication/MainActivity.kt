package xyz.lsy969999.myapplication

import android.app.Activity
import android.app.NativeActivity
import android.content.Context
import android.content.SharedPreferences
import android.os.Bundle
import android.util.Log
import com.google.android.gms.ads.AdError
import com.google.android.gms.ads.AdRequest
import com.google.android.gms.ads.FullScreenContentCallback
import com.google.android.gms.ads.LoadAdError
import com.google.android.gms.ads.MobileAds
import com.google.android.gms.ads.interstitial.InterstitialAd
import com.google.android.gms.ads.interstitial.InterstitialAdLoadCallback;
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch

class MainActivity : NativeActivity() {
    private lateinit var spf: SharedPreferences
    private lateinit var admobInterstitialAd: AdmobInterstitial;
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        spf = getSharedPreferences("kv", Context.MODE_PRIVATE)
        //
        val backgroundScope = CoroutineScope(Dispatchers.IO)
        backgroundScope.launch {
            // Initialize the Google Mobile Ads SDK on a background thread.
            MobileAds.initialize(this@MainActivity) {}
        }
        admobInterstitialAd = AdmobInterstitial()
    }
    fun ffiKvGet(key: String): String {
        val s = spf.getString(key, "");
        return s.toString();
    }
    fun ffiKvSet(key: String, value: String) {
        val editor = spf.edit();
        editor.putString(key, value).apply();
    }

    fun ffiKvDelete(key: String) {
        val editor = spf.edit();
        editor.remove(key).apply();
    }

    fun ffiKvExists(key: String): Boolean {
        return spf.contains(key);
    }

    fun ffiAdmobInterstitialLoad() {
        admobInterstitialAd.load(this, "ca-app-pub-3940256099942544/1033173712");
    }

    fun ffiAdmobInterstitialShow() {
        admobInterstitialAd.show(this)
    }
}

class RustBinding {
    companion object {
        init {
            System.loadLibrary("android");
        }

//        @JvmStatic
//        external fun ffi_greet_to_rust();

//        @JvmStatic
//        external fun ffi_ad_dismiss();
    }
}

class AdmobInterstitial {
    private var mInterstitialAd: InterstitialAd? = null
    private final val TAG = "AdmobInterstitial"

    fun load(ctx: Activity, adId: String) {
        ctx.runOnUiThread {
            val adRequest = AdRequest.Builder().build()
            InterstitialAd.load(ctx,adId, adRequest, object : InterstitialAdLoadCallback() {
                override fun onAdFailedToLoad(adError: LoadAdError) {
                    Log.d(TAG, adError?.toString() ?: "")
                    mInterstitialAd = null
                }

                override fun onAdLoaded(interstitialAd: InterstitialAd) {
                    Log.d(TAG, "Ad was loaded.")
                    mInterstitialAd = interstitialAd
                    mInterstitialAd?.fullScreenContentCallback = object: FullScreenContentCallback() {
                        override fun onAdClicked() {
                            // Called when a click is recorded for an ad.
                            Log.d(TAG, "Ad was clicked.")
                        }

                        override fun onAdDismissedFullScreenContent() {
                            // Called when ad is dismissed.
                            Log.d(TAG, "Ad dismissed fullscreen content.")
                            mInterstitialAd = null
                        }

                        override fun onAdFailedToShowFullScreenContent(p0: AdError) {
                            // Called when ad fails to show.
                            Log.e(TAG, "Ad failed to show fullscreen content.")
                            mInterstitialAd = null
                        }

                        override fun onAdImpression() {
                            // Called when an impression is recorded for an ad.
                            Log.d(TAG, "Ad recorded an impression.")
                        }

                        override fun onAdShowedFullScreenContent() {
                            // Called when ad is shown.
                            Log.d(TAG, "Ad showed fullscreen content.")
                        }
                    }
                }
            })
        }
    }

    fun show(act: Activity) {
        act.runOnUiThread {
            if (mInterstitialAd != null) {
                mInterstitialAd?.show(act)
            } else {
                Log.d("TAG", "The interstitial ad wasn't ready yet.")
            }
        }
    }

    fun isReady(): Boolean {
        return mInterstitialAd != null;
    }

    fun clear() {
        mInterstitialAd = null
    }
}