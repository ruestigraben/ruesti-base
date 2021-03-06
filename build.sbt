organization := "dev.ruestigraben"
name := "ruesti-base"
version := "0.1.0-SNAPSHOT"

autoScalaLibrary := false

enablePlugins(RuestiPlugin)

libraryDependencies ++= Seq(
  "org.slf4j" % "slf4j-api" % "1.7.26",
  "org.scalameta" %% "munit" % "0.7.19" % "test",
  "ch.qos.logback" % "logback-classic" % "1.2.3" % "test"
)

testFrameworks += new TestFramework("munit.Framework")

Test / buildInfoKeys := Seq[BuildInfoKey](
  Test / ruestiTargetName
)

Test / buildInfoPackage := "dev.ruestigraben.base.test"

sbtbuildinfo.BuildInfoPlugin.buildInfoDefaultSettings
sbtbuildinfo.BuildInfoPlugin.buildInfoScopedSettings(Test)
