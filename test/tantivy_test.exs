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

  test "autocomplete", %{resource: resource} do
    assert :ok == Tantivy.add_entry(
      resource,
      "The Old Man and the Sea",
      "He was an old man who fished alone in a skiff in the Gulf Stream and "
    )

    assert Tantivy.search(resource, "fish") == [["The Old Man and the Sea"]]
  end
end
