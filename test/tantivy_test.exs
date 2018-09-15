defmodule TantivyTest do
  use ExUnit.Case

  setup do
    {:ok, resource} = Tantivy.init()
    {:ok, resource: resource}
  end

  test "it works", %{resource: resource} do
    assert :ok == Tantivy.add_entry(resource, "old man and a dog", "whatev")
    assert Tantivy.search(resource, "dog") == [["old man and a dog"]]
  end
end
