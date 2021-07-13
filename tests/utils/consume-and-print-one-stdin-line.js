var stdin = WScript.StdIn;
var stdout = WScript.StdOut;

if (stdin.AtEndOfStream) {
  stdout.WriteLine("END");
  WScript.Quit(1);
}

stdout.WriteLine(stdin.ReadLine());
