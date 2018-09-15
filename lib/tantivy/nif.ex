defmodule Tantivy.NIF do
  @moduledoc false
  use Rustler, otp_app: :tantivy, crate: :tantivy_nif

  @type resource :: reference

  @spec init :: {:ok, resource} | {:error, reason :: any}
  def init do
    error()
  end

  # TODO
  @spec search(resource, String.t()) :: [[String.t()]]
  def search(_resource, _query) do
    error()
  end

  @spec add_entry(resource, String.t(), String.t()) :: :ok | {:error, reason :: any}
  def add_entry(_resource, _title, _body) do
    error()
  end

  defp error do
    :erlang.nif_error(:nif_not_loaded)
  end
end
