package xyz.lsy969999.luckyball

import android.app.Activity
import android.app.NativeActivity
import android.content.Context
import android.content.SharedPreferences
import android.graphics.PixelFormat
import android.os.Build
import android.os.Bundle
import android.util.Log
import android.view.Gravity
import android.view.WindowManager
import android.view.WindowMetrics
import android.widget.FrameLayout
import androidx.core.splashscreen.SplashScreen.Companion.installSplashScreen
//import com.google.android.gms.ads.AdError
//import com.google.android.gms.ads.AdListener
//import com.google.android.gms.ads.AdRequest
//import com.google.android.gms.ads.AdSize
//import com.google.android.gms.ads.AdView
//import com.google.android.gms.ads.FullScreenContentCallback
//import com.google.android.gms.ads.LoadAdError
//import com.google.android.gms.ads.MobileAds
//import com.google.android.gms.ads.interstitial.InterstitialAd
//import com.google.android.gms.ads.interstitial.InterstitialAdLoadCallback;
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import xyz.lsy969999.luckyball.databinding.ActivityMainBinding
import java.util.Locale
import java.util.TimeZone

class MainActivity : NativeActivity() {
    private lateinit var spf: SharedPreferences
//    private lateinit var admobInterstitialAd: AdmobInterstitial;
//    private lateinit var admobBannerAd: AdmobBanner;
//    private lateinit var binding: ActivityMainBinding
    private var isSplashKeep: Boolean = true;
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
//        binding = ActivityMainBinding.inflate(layoutInflater)
//        setContentView(binding.root)
        val splashScreen = installSplashScreen()
        splashScreen.setKeepOnScreenCondition {
            isSplashKeep
        }

        spf = getSharedPreferences("kv", Context.MODE_PRIVATE)


//        admobInterstitialAd = AdmobInterstitial()
//        admobBannerAd = AdmobBanner()
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
//        admobInterstitialAd.load(this, "ca-app-pub-3940256099942544/1033173712");
    }

    fun ffiAdmobInterstitialShow() {
//        admobInterstitialAd.show(this)
    }

    fun ffiAdmobInterstitialIsReady() {
//        val isReady = admobInterstitialAd.isReady()
//        RustBinding.ffi_callback_admob_interstitial_is_ready(isReady)
    }

    fun ffiAdmobInterstitialClear() {
//        admobInterstitialAd.clear()
    }

    fun ffiAppInit() {
//        val backgroundScope = CoroutineScope(Dispatchers.IO)
//        backgroundScope.launch {
//            // Initialize the Google Mobile Ads SDK on a background thread.
//            MobileAds.initialize(this@MainActivity) {
//                RustBinding.ffi_callback_app_init_end()
//            }
//        }
    }

    fun ffiSplashHide() {
        CoroutineScope(Dispatchers.Main).launch {
            delay(500L)  // 0.5초 대기
            isSplashKeep = false
        }
    }

    fun ffiGetCurrentEpochTime(): Long {
        val currentEpochTime = System.currentTimeMillis() / 1000
        return currentEpochTime;
    }

    fun fiiGetLocale(): String {
        val locale = Locale.getDefault()
        val languageCode = locale.language // 언어 코드 (예: ko)
        val countryCode = locale.country
        return "${languageCode}_${countryCode}"
    }

    fun ffiGetTimeOffset(): Int {
        val timeZone = TimeZone.getDefault()
        val offsetInMillis = timeZone.rawOffset  // UTC와의 시간 오프셋 (밀리초 단위)
        val offsetInSecs = offsetInMillis / 1000    // 시간 단위로 변환
//        print("offsetInSec: $offsetInSecs")
        return offsetInSecs
    }

    fun ffiAdmobBannerLaunch() {
//        admobBannerAd.launch(this, binding)
    }
}

class RustBinding {
    companion object {
        init {
            System.loadLibrary("android");
        }

        @JvmStatic
        external fun ffi_callback_admob_interstitial_load_success();
        @JvmStatic
        external fun ffi_callback_admob_interstitial_load_fail(errMsg: String);
        @JvmStatic
        external fun ffi_callback_admob_interstitial_showed();
        @JvmStatic
        external fun ffi_callback_admob_interstitial_show_fail(errMsg: String);
        @JvmStatic
        external fun ffi_callback_admob_interstitial_dismissed();

        @JvmStatic
        external fun ffi_callback_admob_interstitial_is_ready(isReady: Boolean);

        @JvmStatic
        external fun ffi_callback_app_init_end();
    }
}

//class AdmobBanner {
//    private final val TAG = "AdmobBanner"
//    private var adView: AdView? = null
//    private var adViewContainer: FrameLayout? = null
//
//    fun launch(act: Activity, binding: ActivityMainBinding) {
//        Log.d(TAG, "launch: ")
//        act.runOnUiThread {
////            createFrameLayout(act);
//            val adView = AdView(act);
//            adView.adUnitId = "ca-app-pub-3940256099942544/9214589741";
//            adView.setAdSize(adSize(act))
//            this.adView = adView
//
//            // WindowManager를 통해 최상단에 추가
//            val windowManager = act.getSystemService(Context.WINDOW_SERVICE) as WindowManager
//            val params = WindowManager.LayoutParams(
//                WindowManager.LayoutParams.MATCH_PARENT,
//                WindowManager.LayoutParams.WRAP_CONTENT,
//                WindowManager.LayoutParams.TYPE_APPLICATION_PANEL, // 앱 내에서만 표시
//                WindowManager.LayoutParams.FLAG_NOT_FOCUSABLE,      // 포커스 방지, 다른 뷰와 상호작용 가능
//                PixelFormat.TRANSLUCENT
//            )
//            params.gravity = Gravity.BOTTOM or Gravity.CENTER_HORIZONTAL
//            windowManager.addView(adView, params)
////            binding.adViewContainer.removeAllViews()
////            binding.adViewContainer.addView(adView)
////            binding.adViewContainer.bringToFront()
////            adView.bringToFront()
//            val adRequest = AdRequest.Builder().build()
//            adView.loadAd(adRequest)
//
//            adView.adListener = object : AdListener() {
//                override fun onAdClicked() {
//                    super.onAdClicked()
//                    Log.d(TAG, "onAdClicked: ")
//                }
//
//                override fun onAdClosed() {
//                    super.onAdClosed()
//                    Log.d(TAG, "onAdClosed: ")
//                }
//
//                override fun onAdFailedToLoad(p0: LoadAdError) {
//                    super.onAdFailedToLoad(p0)
//                    Log.d(TAG, "onAdFailedToLoad: ${p0}")
//                }
//
//                override fun onAdImpression() {
//                    super.onAdImpression()
//                    Log.d(TAG, "onAdImpression: ")
//                }
//
//                override fun onAdLoaded() {
//                    super.onAdLoaded()
//                    Log.d(TAG, "onAdLoaded: ")
//                    val w = binding.adViewContainer.width
//                    val h = binding.adViewContainer.height
//                    val w2 = adView.width
//                    val h2 = adView.height
//                    Log.d(TAG, "onAdLoaded: wh: $w, $h, adview wh: $w2, $h2")
//                }
//
//                override fun onAdOpened() {
//                    super.onAdOpened()
//                    Log.d(TAG, "onAdOpened: ")
//                }
//
//                override fun onAdSwipeGestureClicked() {
//                    super.onAdSwipeGestureClicked()
//                    Log.d(TAG, "onAdSwipeGestureClicked: ")
//                }
//
//            }
//        }
//    }
//
//    fun adSize(act: Activity): AdSize {
//        val displayMetrics = act.resources.displayMetrics
//        val adWidthPixels =
//            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
//                val windowMetrics: WindowMetrics = act.windowManager.currentWindowMetrics
//                windowMetrics.bounds.width()
//            } else {
//                displayMetrics.widthPixels
//            }
//        val density = displayMetrics.density
//        val adWidth = (adWidthPixels / density).toInt()
//        val adSize = AdSize.getCurrentOrientationAnchoredAdaptiveBannerAdSize(act, adWidth);
//        Log.d(TAG, "adSize: $adSize")
//        return adSize
//    }
//
//    fun createFrameLayout(act: Activity) {
//        act.runOnUiThread {
//            adViewContainer = FrameLayout(act);
//            adViewContainer?.layoutParams = FrameLayout.LayoutParams(
//                FrameLayout.LayoutParams.MATCH_PARENT,
//                FrameLayout.LayoutParams.WRAP_CONTENT,
//            )
//            act.setContentView(adViewContainer);
//            Log.d(TAG, "launch: createFrameLayout")
//        }
//    }
//}

//class AdmobInterstitial {
//    private var mInterstitialAd: InterstitialAd? = null
//    private final val TAG = "AdmobInterstitial"
//
//    fun load(ctx: Activity, adId: String) {
//        ctx.runOnUiThread {
//            val adRequest = AdRequest.Builder().build()
//            InterstitialAd.load(ctx,adId, adRequest, object : InterstitialAdLoadCallback() {
//                override fun onAdFailedToLoad(adError: LoadAdError) {
//                    val errMsg = adError.message;
//                    Log.d(TAG, "onAdFailedToLoad errMsg: $errMsg");
//                    RustBinding.ffi_callback_admob_interstitial_load_fail(errMsg);
//                    mInterstitialAd = null
//                }
//
//                override fun onAdLoaded(interstitialAd: InterstitialAd) {
//                    Log.d(TAG, "Ad was loaded.")
//                    RustBinding.ffi_callback_admob_interstitial_load_success();
//                    mInterstitialAd = interstitialAd
//                    mInterstitialAd?.fullScreenContentCallback = object: FullScreenContentCallback() {
//                        override fun onAdClicked() {
//                            // Called when a click is recorded for an ad.
//                            Log.d(TAG, "Ad was clicked.")
//                        }
//
//                        override fun onAdDismissedFullScreenContent() {
//                            // Called when ad is dismissed.
//                            Log.d(TAG, "Ad dismissed fullscreen content.")
//                            RustBinding.ffi_callback_admob_interstitial_dismissed()
//                            mInterstitialAd = null
//                        }
//
//                        override fun onAdFailedToShowFullScreenContent(p0: AdError) {
//                            // Called when ad fails to show.
//                            val errMsg = p0.message;
//                            Log.e(TAG, "Ad failed to show fullscreen content. errMsg: $errMsg")
//                            RustBinding.ffi_callback_admob_interstitial_show_fail(errMsg)
//                            mInterstitialAd = null
//                        }
//
//                        override fun onAdImpression() {
//                            // Called when an impression is recorded for an ad.
//                            Log.d(TAG, "Ad recorded an impression.")
//                        }
//
//                        override fun onAdShowedFullScreenContent() {
//                            // Called when ad is shown.
//                            Log.d(TAG, "Ad showed fullscreen content.")
//                            RustBinding.ffi_callback_admob_interstitial_showed()
//                        }
//                    }
//                }
//            })
//        }
//    }
//
//    fun show(act: Activity) {
//        act.runOnUiThread {
//            if (mInterstitialAd != null) {
//                mInterstitialAd?.show(act)
//            } else {
//                Log.d("TAG", "The interstitial ad wasn't ready yet.")
//            }
//        }
//    }
//
//    fun isReady(): Boolean {
//        return mInterstitialAd != null;
//    }
//
//    fun clear() {
//        mInterstitialAd = null
//    }
//}