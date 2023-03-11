using System.Diagnostics;

namespace HelloWorld
{
    class Hello
    {
        static void Main(string[] args)
        {
            var stopWatch = Stopwatch.StartNew();
            long nanosecPerTick = (1000L*1000L*1000L) / Stopwatch.Frequency;
            var maxPrime = 1000;
            if (args.Length > 0 && args[0] != "") {
                maxPrime = int.Parse(args[0]);
            }
            var primes = new List<int>();
            foreach (int i in Enumerable.Range(1, maxPrime))
            {
                Console.Write("Is {0} prime? ", i);
                if (isPrime(i, primes)) {
                    primes.Add(i);
                    Console.Write("yes\r");
                }
                else
                {
                    Console.Write("no\r");
                }
            }
            stopWatch.Stop();
            long nanos = stopWatch.ElapsedTicks * nanosecPerTick;
            var span = stopWatch.Elapsed;
            Console.WriteLine("\nLen: {0}", primes.Count);
            Console.WriteLine("Time (ns): {0}", nanos);
            Console.WriteLine("Time (µs): {0}", nanos / 1000.0);
            Console.WriteLine("Time (ms): {0}", span.Milliseconds);
            Console.WriteLine("Time (s): {0}", span.Milliseconds / 1000.0);

        }
        static Boolean isPrime(int value, List<int> primes) {
            foreach (int p in primes)
            {
                if (p == 1 || p == value) {
                    continue;
                }
                if (value % p == 0) {
                    return false;
                }
            }
            return true;
        }
    }
}