import ai.kien.python.Python
import com.keithalcock.sbt.BuildUtils

lazy val javaOpts = {
  if (BuildUtils.isWindows())
    // Fill this in manually.
    Seq(
      "-Djna.library.path=D:/ProgramFiles/Python39",
      "-Dscalapy.python.library=python39"
    )
  else {
    val python = if (BuildUtils.isMac()) Python("python3.10") else Python()
    val result = python.scalapyProperties.get.map { case (key, value) =>
      s"-D$key=$value"
    }.toSeq
    result.foreach { result => println(s"Python: $result") }
    result
  }
}

ThisBuild / run / javaOptions ++= javaOpts
ThisBuild / Test / javaOptions ++= javaOpts
