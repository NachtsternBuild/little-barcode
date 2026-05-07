import java.awt.image.BufferedImage;
import java.io.File;
import java.io.FileOutputStream;
import java.util.Scanner;

import javax.imageio.ImageIO;

import org.krysalis.barcode4j.impl.code128.Code128Bean;
import org.krysalis.barcode4j.output.bitmap.BitmapCanvasProvider;

import com.google.zxing.*;
import com.google.zxing.client.j2se.*;
import com.google.zxing.common.BitMatrix;

public class QRCodeandBarcode {

    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);

        while (true) {
            System.out.println("\n====== MENÜ ======");
            System.out.println("1 - Text → Barcode & QR-Code erzeugen");
            System.out.println("2 - Code einlesen");
            System.out.println("0 - Beenden");
            System.out.print("Auswahl: ");

            String choice = scanner.nextLine();

            try {
                switch (choice) {
                    case "1":
                        generate(scanner);
                        break;
                    case "2":
                        read(scanner);
                        break;
                    case "0":
                        System.out.println("Programm beendet.");
                        scanner.close();
                        return;
                    default:
                        System.out.println("Ungültige Eingabe!");
                }
            } catch (Exception e) {
                System.out.println("Fehler: " + e.getMessage());
            }
        }
    }

    // TEXT → CODES
    public static void generate(Scanner scanner) throws Exception {
        System.out.print("Text eingeben: ");
        String text = scanner.nextLine();

        String barcodeFile = "barcode.png";
        String qrFile = "qrcode.png";

        // Barcode
        Code128Bean bean = new Code128Bean();
        try (FileOutputStream out = new FileOutputStream(new File(barcodeFile))) {
            BitmapCanvasProvider canvas = new BitmapCanvasProvider(
                    out, "image/png", 160, BufferedImage.TYPE_BYTE_BINARY, false, 0);
            bean.generateBarcode(canvas, text);
            canvas.finish();
        }

        // QR-Code
        BitMatrix matrix = new MultiFormatWriter().encode(
                text, BarcodeFormat.QR_CODE, 300, 300);

        MatrixToImageWriter.writeToPath(matrix, "PNG", new File(qrFile).toPath());

        System.out.println("Erstellt: " + barcodeFile + " & " + qrFile);
    }

    // CODE → TEXT
    public static void read(Scanner scanner) throws Exception {
        System.out.print("Pfad zum Bild: ");
        String path = scanner.nextLine();

        BufferedImage image = ImageIO.read(new File(path));

        if (image == null) {
            System.out.println("Bild konnte nicht geladen werden!");
            return;
        }

        LuminanceSource source = new BufferedImageLuminanceSource(image);
        BinaryBitmap bitmap = new BinaryBitmap(new HybridBinarizer(source));

        Result result = new MultiFormatReader().decode(bitmap);

        System.out.println("Gelesener Text: " + result.getText());
    }
}import java.awt.image.BufferedImage;
import java.io.File;
import java.io.FileOutputStream;
import java.util.Scanner;

import javax.imageio.ImageIO;

import org.krysalis.barcode4j.impl.code128.Code128Bean;
import org.krysalis.barcode4j.output.bitmap.BitmapCanvasProvider;

import com.google.zxing.*;
        import com.google.zxing.client.j2se.*;
        import com.google.zxing.common.BitMatrix;

public class QRCodeandBarcode {

    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);

        while (true) {
            System.out.println("\n====== MENÜ ======");
            System.out.println("1 - Text → Barcode & QR-Code erzeugen");
            System.out.println("2 - Code einlesen");
            System.out.println("0 - Beenden");
            System.out.print("Auswahl: ");

            String choice = scanner.nextLine();

            try {
                switch (choice) {
                    case "1":
                        generate(scanner);
                        break;
                    case "2":
                        read(scanner);
                        break;
                    case "0":
                        System.out.println("Programm beendet.");
                        scanner.close();
                        return;
                    default:
                        System.out.println("Ungültige Eingabe!");
                }
            } catch (Exception e) {
                System.out.println("Fehler: " + e.getMessage());
            }
        }
    }

    // TEXT → CODES
    public static void generate(Scanner scanner) throws Exception {
        System.out.print("Text eingeben: ");
        String text = scanner.nextLine();

        String barcodeFile = "barcode.png";
        String qrFile = "qrcode.png";

        // Barcode
        Code128Bean bean = new Code128Bean();
        try (FileOutputStream out = new FileOutputStream(new File(barcodeFile))) {
            BitmapCanvasProvider canvas = new BitmapCanvasProvider(
                    out, "image/png", 160, BufferedImage.TYPE_BYTE_BINARY, false, 0);
            bean.generateBarcode(canvas, text);
            canvas.finish();
        }

        // QR-Code
        BitMatrix matrix = new MultiFormatWriter().encode(
                text, BarcodeFormat.QR_CODE, 300, 300);

        MatrixToImageWriter.writeToPath(matrix, "PNG", new File(qrFile).toPath());

        System.out.println("Erstellt: " + barcodeFile + " & " + qrFile);
    }

    // CODE → TEXT
    public static void read(Scanner scanner) throws Exception {
        System.out.print("Pfad zum Bild: ");
        String path = scanner.nextLine();

        BufferedImage image = ImageIO.read(new File(path));

        if (image == null) {
            System.out.println("Bild konnte nicht geladen werden!");
            return;
        }

        LuminanceSource source = new BufferedImageLuminanceSource(image);
        BinaryBitmap bitmap = new BinaryBitmap(new HybridBinarizer(source));

        Result result = new MultiFormatReader().decode(bitmap);

        System.out.println("Gelesener Text: " + result.getText());
    }
}