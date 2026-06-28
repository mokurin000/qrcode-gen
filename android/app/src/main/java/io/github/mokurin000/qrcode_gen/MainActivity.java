package io.github.mokurin000.qrcode_gen;

import android.os.Build;
import android.view.RoundedCorner;
import android.view.WindowInsets;

import rs.compio.winio.Activity;

public class MainActivity extends Activity {

    static {
        System.loadLibrary("main");
    }

    /**
     * @return screen bottom left rounded corner radius
     */
    public int getBottomLeftCornerRadius() {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.S) {
            return 0;
        }

        WindowInsets insets
                = getWindow().getDecorView().getRootWindowInsets();

        if (insets == null) {
            return 0;
        }

        RoundedCorner corner
                = insets.getRoundedCorner(RoundedCorner.POSITION_BOTTOM_LEFT);

        return corner != null ? corner.getRadius() : 0;
    }
}
