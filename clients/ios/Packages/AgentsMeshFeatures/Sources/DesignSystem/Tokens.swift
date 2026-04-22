import SwiftUI

/// Design tokens aligned with `clients/web/src/app/globals.css` (GitHub
/// pastel palette). Kept minimal — richer styling added incrementally.
public enum AMColors {
    public static let background = Color(red: 0.98, green: 0.98, blue: 0.98)
    public static let foreground = Color(red: 0.14, green: 0.16, blue: 0.18)
    public static let mutedForeground = Color(red: 0.45, green: 0.48, blue: 0.52)
    public static let border = Color(red: 0.86, green: 0.88, blue: 0.90)
    public static let primary = Color(red: 0.13, green: 0.36, blue: 0.83)
    public static let primaryForeground = Color.white
    public static let card = Color.white
    public static let destructive = Color(red: 0.82, green: 0.28, blue: 0.28)
}

public enum AMSpacing {
    public static let xs: CGFloat = 4
    public static let s: CGFloat = 8
    public static let m: CGFloat = 12
    public static let l: CGFloat = 16
    public static let xl: CGFloat = 24
    public static let xxl: CGFloat = 32
}

public enum AMRadius {
    public static let s: CGFloat = 6
    public static let m: CGFloat = 8
    public static let l: CGFloat = 12
}

public enum AMTypography {
    public static let title = Font.system(size: 28, weight: .bold)
    public static let heading = Font.system(size: 20, weight: .semibold)
    public static let body = Font.system(size: 15)
    public static let caption = Font.system(size: 13)
    public static let mono = Font.system(size: 14, design: .monospaced)
}
