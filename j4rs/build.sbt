name := "tokenizer-j4rs"
description := ""

resolvers ++= Seq(
//  Resolvers.localResolver, // Reserve for Two Six.
//  Resolvers.clulabResolver // processors-models, transitive dependency
)

libraryDependencies ++= {
  Seq(
    "io.github.astonbitecode" % "j4rs" % "0.13.0"
  )
}

fork := true

// assembly / mainClass := Some("com.keithalcock.tokenizer.scalapy.apps.ExampleApp")
