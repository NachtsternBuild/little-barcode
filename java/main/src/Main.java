import java.util.Scanner;

public class Main {

    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);

        // --- TEIL 1: Deine Regex-Tests ---
        runRegexTests();

        // --- TEIL 2: Der interaktive Barcode/QR-Generator ---
        System.out.println("\n--- STARTING BARCODE TOOL ---");
        boolean running = true;
        while (running) {
            System.out.println("\n====== HAUPTMENÜ ======");
            System.out.println("1 - Text → Barcode & QR-Code erzeugen");
            System.out.println("2 - Code einlesen");
            System.out.println("0 - Beenden");
            System.out.print("Auswahl: ");

            String choice = scanner.nextLine();

            try {
                switch (choice) {
                    case "1":
                        QRCodeandBarcode.generate(scanner);
                        break;
                    case "2":
                        QRCodeandBarcode.read(scanner);
                        break;
                    case "0":
                        System.out.println("Programm beendet.");
                        running = false;
                        break;
                    default:
                        System.out.println("Ungültige Eingabe!");
                }
            } catch (Exception e) {
                System.out.println("Fehler: " + e.getMessage());
                e.printStackTrace(); // Hilfreich für Debugging in IntelliJ
            }
        }
        scanner.close();
    }

    private static void runRegexTests() {
        System.out.println("----- TEST IDENTIFYCONTENT -----");
        String input1 = "http://dhsn.de";
        String input2 = "9781234567890";
        String input3 = "Ich bin auch gültig";

        System.out.println("Input1 ist ein " + RegexValidator.identifyContent(input1));
        System.out.println("Input2 ist ein " + RegexValidator.identifyContent(input2));
        System.out.println("Input3 ist ein " + RegexValidator.identifyContent(input3));

        // Hier kannst du weitere Tests aus deinem alten Code einfügen...
        System.out.println("----- TEST WLAN -----");
        String testWLAN = "WIFI:S:MeinNetzwerk;P:Passwort123;;";
        System.out.println("WLAN Name: " + RegexValidator.extractWifi(testWLAN));
    }
}