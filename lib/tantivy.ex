defmodule Tantivy do
  @moduledoc """
  Documentation for Tantivy.
  """

  defdelegate init, to: __MODULE__.NIF
  defdelegate search(resource, query), to: __MODULE__.NIF
  defdelegate add_entry(resource, title, body), to: __MODULE__.NIF
  defdelegate add_entries(resource, docs), to: __MODULE__.NIF
  defdelegate explain(resource, query), to: __MODULE__.NIF
end
