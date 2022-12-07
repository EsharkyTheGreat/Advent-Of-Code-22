
void main(){
  import std.file : readText;
  import std.stdio : writeln;
  try{
    string text = readText("input.txt");

    import std.string : splitLines;
    auto lines = text.splitLines();

    import std.conv : to;

    int maximum = 0;
    int sum = 0;

    foreach(line;lines){
      if(line == ""){
        maximum = maximum>=sum?maximum:sum;
        sum = 0;
      }
      else
        sum += to!int(line);
    }
    maximum = maximum>=sum?maximum:sum;
    writeln(maximum);
  }
  catch(Exception E){
    writeln(E);
  }

}
