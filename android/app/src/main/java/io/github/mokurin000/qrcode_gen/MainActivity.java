package io.github.mokurin000.qrcode_gen;

import android.os.Build;
import android.view.RoundedCorner;
import rs.compio.winio.Activity;

public class MainActivity extends Activity {

    static {
        System.loadLibrary("main");
    }
}
