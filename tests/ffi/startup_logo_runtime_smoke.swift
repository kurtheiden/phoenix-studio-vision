import AppKit
import SwiftUI

@main struct StartupLogoSmoke {
    @MainActor static func main() throws {
        guard CommandLine.arguments.count == 2,
              let bundle = Bundle(path: CommandLine.arguments[1]),
              let resource = bundle.url(forResource: PhoenixStartupPresentation.logoResource, withExtension: "png"),
              let image = PhoenixStartupPresentation.loadImage(bundle: bundle),
              let bitmap = NSBitmapImageRep(data: try Data(contentsOf: resource)) else {
            fatalError("Built application startup-logo lookup/decode failed")
        }
        precondition(bitmap.pixelsWide == 1024 && bitmap.pixelsHigh == 1024 && bitmap.hasAlpha)
        var transparent = false
        for y in 0..<bitmap.pixelsHigh {
            for x in 0..<bitmap.pixelsWide {
                transparent = transparent || bitmap.colorAt(x: x, y: y)!.alphaComponent == 0
            }
        }
        precondition(transparent, "Runtime artwork lost transparency")
        // Exercise the same decoded-image initializer and aspect-fit treatment as startup.
        let renderer = ImageRenderer(content: Image(nsImage: image).resizable().scaledToFit()
            .frame(width: 220, height: 220))
        guard let rendered = renderer.nsImage,
              let data = rendered.tiffRepresentation,
              let output = NSBitmapImageRep(data: data) else { fatalError("Startup rendering failed") }
        var visible = false
        for y in 0..<output.pixelsHigh {
            for x in 0..<output.pixelsWide {
                visible = visible || output.colorAt(x: x, y: y)!.alphaComponent > 0.01
            }
        }
        precondition(visible, "Startup image rendered an empty frame")
        print("Built-app startup logo lookup/decode/render PASS: 1024 x 1024, alpha and visible pixels")
    }
}
