organization := "dev.ruestigraben"
name := "ruesti-base"
version := "0.1.0-SNAPSHOT"

autoScalaLibrary := false

enablePlugins(RuestiPlugin)

libraryDependencies ++= Seq(
  "org.slf4j" % "slf4j-api" % "1.7.26",
  "com.lihaoyi" %% "utest" % "0.7.1" % "test",
  "ch.qos.logback" % "logback-classic" % "1.2.3" % "test"
)

testFrameworks += new TestFramework("utest.runner.Framework")
