{:ok, resource} = Tantivy.init()

[
  %{
    title: "The Old Man and the Sea",
    body:
      "He was an old man who fished alone in a skiff in the Gulf Stream and " <>
        "he had gone eighty-four days now without taking a fish."
  },
  %{
    title: "Of Mice and Men",
    body:
      "A few miles south of Soledad, the Salinas River drops in close to the hillside " <>
        "ank and runs deep and green. The water is warm too, for it has slipped twinkling " <>
        "over the yellow sands in the sunlight before reaching the narrow pool. On one " <>
        "side of the river the golden foothill slopes curve up to the strong and rocky " <>
        "Gabilan Mountains, but on the valley side the water is lined with trees—willows " <>
        "fresh and green with every spring, carrying in their lower leaf junctures the " <>
        "debris of the winter’s flooding; and sycamores with mottled, white, recumbent " <>
        "limbs and branches that arch over the pool"
  },
  %{
    title: "Frankenstein",
    body:
      "You will rejoice to hear that no disaster has accompanied the commencement of an " <>
        "enterprise which you have regarded with such evil forebodings. I arrived here " <>
        "yesterday, and my first task is to assure my dear sister of my welfare and " <>
        "increasing confidence in the success of my undertaking."
  }
] |> Enum.each(fn entry -> Tantivy.add_entry(resource, entry.title, entry.body) end)
Tantivy.search(resource, "fish")

Tantivy.explain(resource, "fish")
