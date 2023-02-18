import java.time.Duration;
import java.time.Instant;
import java.time.temporal.ChronoUnit;
import java.util.ArrayList;

public class Main {

    public static void main(String[] args) {
        long start = System.nanoTime();
        ArrayList<Integer> primes = new ArrayList<Integer>();
        int largestNumber = 1000;
        if (args.length > 0 && !args[0].isEmpty()) {
            largestNumber = Integer.parseInt(args[0]);
        }
        for (int i = 1; i <= largestNumber; i++) {
            System.out.print("Is " + i + " prime? ");
            if (isPrime(i, primes)) {
                System.out.print("yes\r");
                primes.add(i);
            }
            else {
                System.out.print("no \r");
            }
        }
        long end = System.nanoTime();
        Duration diff = Duration.of(end - start, ChronoUnit.NANOS);
        long nanos = diff.toNanos();
        System.out.println("\nCount: " + primes.size());
        System.out.println("Time (ns): " + nanos);
        System.out.println("Time (µs): " + String.format("%.3f", nanos / 1_000.0));
        System.out.println("Time (ms): " + String.format("%.3f", nanos / 1_000_000.0));
        System.out.println("Time (s):  " + String.format("%.4f", nanos / 1_000_000_000.0));
    }

    public static boolean isPrime(int i, ArrayList<Integer> primes) {
        if ((i == 1) || (i == 2)) {
            return true;
        }
        ArrayList<Integer> divs = new ArrayList<Integer>();
        for (int p : primes) {
            if (i % p == 0) {
                divs.add(p);
            }
        }
        return (divs.size() == 1 && divs.get(0) == 1);
    }
}
