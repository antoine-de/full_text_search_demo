defmodule Tantivy.MixProject do
  use Mix.Project

  def project do
    [
      app: :tantivy,
      version: "0.1.0",
      elixir: "~> 1.7",
      start_permanent: Mix.env() == :prod,
      compilers: [:rustler | Mix.compilers()],
      rustler_crates: rustler_crates(),
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger],
      mod: {Tantivy.Application, []}
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.20"},
      {:benchee, "~> 0.13", only: :bench}
    ]
  end

  defp rustler_crates do
    [tantivy_nif: [path: "native/tantivy_nif", mode: rustc_mode(Mix.env())]]
  end

  defp rustc_mode(env) when env in [:prod, :bench], do: :release
  defp rustc_mode(_env), do: :debug
end
