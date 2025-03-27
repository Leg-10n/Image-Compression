

final class PNG$_ {
def args = PNG_sc.args$
def scriptPath = """PNG.sc"""
/*<script>*/
/*
1.Load the PNG using image crate.

2.Extract raw pixel data.

3.Split into chunks and compress them in parallel using rayon.

4.Use lz77 crate for actual compression.

5.Save the compressed output to a file. 
 */
/*</script>*/ /*<generated>*//*</generated>*/
}

object PNG_sc {
  private var args$opt0 = Option.empty[Array[String]]
  def args$set(args: Array[String]): Unit = {
    args$opt0 = Some(args)
  }
  def args$opt: Option[Array[String]] = args$opt0
  def args$: Array[String] = args$opt.getOrElse {
    sys.error("No arguments passed to this script")
  }

  lazy val script = new PNG$_

  def main(args: Array[String]): Unit = {
    args$set(args)
    val _ = script.hashCode() // hashCode to clear scalac warning about pure expression in statement position
  }
}

export PNG_sc.script as `PNG`

