
void main(){
  import std.file : readText;
  import std.stdio : writeln;
  try{
    string text = readText("input.txt");

    import std.string : splitLines;
    auto lines = text.splitLines();

    import std.conv : to;
    import std.container : DList;

    auto list = DList!int();
    int sum = 0;

    foreach(line;lines){
      if(line == ""){
        list.insertBack(sum);
        sum = 0;
      }
      else
        sum += to!int(line);
    }
    list.insertBack(sum);
    import std.algorithm.sorting: sort;
    import std.algorithm.mutation: reverse ;
    import std.array :array;

    auto x = list.array.sort.reverse;
    writeln(x[0]+x[1]+x[2]);

    
  }
  catch(Exception E){
    writeln(E);
  }

}
