# RAG Eval Report

**KB:** `Poems`  **Model:** `llama3.1:8b`

**Flags:** top_k=5  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 20 |
| Overall recall (token-overlap) | 46.1% (35.0/76) |
| Avg latency | 5557ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | What poetic form and style is most associated with William Blake's Songs of Innocence and Experience, and what central thematic contrast does that title suggest? | 4/4 (100%) | Poems of William Blake.pdf, Poems- Three Series, Complete.pdf, Robert Browning - Complete Poetic and Dramatic Works.html | 25137ms |
| q02 | What central Romantic aesthetic is Keats most known for in his poetry? | 2/3 (67%) | Edgar Allan Poe - The Works of Edgar Allan Poe Raven Edition Volume 5.html, Lord Byron - Index of Project Gutenberg Works.html, Poems by John Keats.pdf, Henry Wadsworth Longfellow - Complete Poetical Works.html | 6408ms |
| q03 | Compare Whitman's free verse style in Leaves of Grass to Dickinson's compressed, slant-rhymed style — what formal element most sharply distinguishes the two? | 3/4 (75%) | Poems- Three Series, Complete.pdf, Leaves of Grass (1891 deathbed edition).pdf | 6037ms |
| q04 | What subject matter does Whitman's Drum-Taps specifically address, and how does this differ thematically from the broader Leaves of Grass? | 2/4 (50%) | Drum-Taps.pdf | 4297ms |
| q05 | Pair: Both Blake and Wordsworth wrote within visionary/Romantic nature traditions — how does Blake's symbolic, mystical approach to nature differ from Wordsworth's plain-style approach? | 4/4 (100%) | William Wordsworth - The Poetical Works Volume 1.html, Poems of William Blake.pdf | 4154ms |
| q06 | Pair: Keats and Byron were both major Romantic poets of the same generation — how does Keats's sensuous aestheticism differ from Byron's satirical, dramatic voice? | 2/4 (50%) | Robert Browning - Complete Poetic and Dramatic Works.html, William Wordsworth - The Poetical Works Volume 1.html, Lord Byron - Index of Project Gutenberg Works.html | 6737ms |
| q07 | Pair: Whitman and Longfellow were both major 19th-century American poets — how does Whitman's radical free verse differ from Longfellow's formal narrative meter? | 1/4 (25%) | Henry Wadsworth Longfellow - Complete Poetical Works.html, Drum-Taps.pdf, Robert Browning - Complete Poetic and Dramatic Works.html | 3679ms |
| q08 | Pair: Dickinson and Poe were both American poets preoccupied with death — how does Dickinson's compressed, metaphysical style differ from Poe's Gothic, musical style? | 2/4 (50%) | Edgar Allan Poe - The Works of Edgar Allan Poe Raven Edition Volume 5.html, Poems- Three Series, Complete.pdf | 4357ms |
| q09 | Pair: Rossetti and Elizabeth Barrett Browning were both major Victorian women poets — how does Rossetti's devotional, sensuous poetry differ from Browning's political, passionate poetry? | 3/4 (75%) | Robert Browning - Complete Poetic and Dramatic Works.html | 4549ms |
| q10 | Pair: Robert Browning and Tennyson were the two dominant male voices of Victorian poetry — how does Browning's dramatic-monologue psychological realism differ from Tennyson's elegiac, formal Poet Laureate style? | 1/4 (25%) | Alfred Lord Tennyson - Index of Project Gutenberg Works.html, Robert Browning - Complete Poetic and Dramatic Works.html | 5685ms |
| q11 | Bonus pair: Robert Browning and Elizabeth Barrett Browning were a real married couple — what stylistic differences distinguish their individual poetic voices despite their personal relationship? | 3/4 (75%) | Robert Browning - Complete Poetic and Dramatic Works.html | 5468ms |
| q12 | What does the "Index of Project Gutenberg Works" format (used for Tennyson and Byron in this cluster) provide, as distinct from a single poetry-collection volume? | 0/4 (0%) | Lord Byron - Index of Project Gutenberg Works.html, Alfred Lord Tennyson - Index of Project Gutenberg Works.html | 2519ms |
| q13 | Which poet in this cluster is most associated with dramatic monologue as a technique, and what does that technique formally involve? | 1/4 (25%) | Robert Browning - Complete Poetic and Dramatic Works.html | 4119ms |
| q14 | Cross-era: How does the plainness valued by Wordsworth's Romantic nature poetry differ from the more self-conscious irony of a Victorian poet like Tennyson or Browning? | 0/4 (0%) | William Wordsworth - The Poetical Works Volume 1.html | 2907ms |
| q15 | Near-miss: Poe and Dickinson are both American and both preoccupied with mortality — what specific formal feature would most reliably distinguish a Poe poem from a Dickinson poem even without an author byline? | 0/4 (0%) | Edgar Allan Poe - The Works of Edgar Allan Poe Raven Edition Volume 5.html, Poems- Three Series, Complete.pdf | 3617ms |
| q16 | Near-miss: Byron and Robert Browning are both known for a theatrical poetic persona — how does Byron's Romantic-era "Byronic hero" differ from Browning's Victorian dramatic-monologue speaker? | 3/4 (75%) | Robert Browning - Complete Poetic and Dramatic Works.html, Lord Byron - Index of Project Gutenberg Works.html | 6175ms |
| q17 | What common Victorian-era social or religious tension does Christina Rossetti's devotional poetry often engage with? | 1/2 (50%) | William Wordsworth - The Poetical Works Volume 1.html, Robert Browning - Complete Poetic and Dramatic Works.html, Christina Rossetti - Poems.html | 3790ms |
| q18 | Cross-document: Of the poets in this cluster, which two are most likely to be confused by a retrieval system due to overlapping themes of death and mortality across different eras? | 0/3 (0%) | Poems- Three Series, Complete.pdf, Henry Wadsworth Longfellow - Complete Poetical Works.html, Leaves of Grass (1891 deathbed edition).pdf, William Wordsworth - The Poetical Works Volume 1.html, Edgar Allan Poe - The Works of Edgar Allan Poe Raven Edition Volume 5.html | 4547ms |
| q19 | Edge case: Longfellow was America's most popular 19th-century poet in his lifetime, while Whitman was initially controversial — what does this contrast suggest about the relationship between formal conventionality and contemporary popularity? | 3/4 (75%) | William Wordsworth - The Poetical Works Volume 1.html, Edgar Allan Poe - The Works of Edgar Allan Poe Raven Edition Volume 5.html, Drum-Taps.pdf, Henry Wadsworth Longfellow - Complete Poetical Works.html | 4547ms |
| q20 | What distinguishes Elizabeth Barrett Browning's "Poetical Works" thematically from the devotional focus found in Christina Rossetti's poems? | 0/4 (0%) | Robert Browning - Complete Poetic and Dramatic Works.html, Christina Rossetti - Poems.html | 2428ms |

## Answers

### q01 — What poetic form and style is most associated with William Blake's Songs of Innocence and Experience, and what central thematic contrast does that title suggest?

The poetic form and style most associated with William Blake's Songs of Innocence and Experience are those of Romanticism and Symbolism. The poems in this collection often feature:

* Simple, lyrical language
* Use of symbolism and metaphor to convey complex ideas and emotions
* Emphasis on the individual's experience and emotions
* Exploration of themes such as innocence, experience, nature, and the human condition

The title "Songs of Innocence and Experience" suggests a central thematic contrast between two states of being: innocence (youthful naivety and purity) and experience (maturity and worldly understanding). The title implies that these poems will explore the transition from one state to the other, and the ways in which innocence is lost or transformed through experience.

The provided sources do not explicitly describe the poetic form and style as Romanticism and Symbolism, but it can be inferred based on the language and themes present in the excerpts.

### q02 — What central Romantic aesthetic is Keats most known for in his poetry?

The provided sources do not contain information about a specific central Romantic aesthetic that Keats is most known for. However, [4] mentions "the graceful insouciance of its metre" and the ease of the general manner as notable aspects of Keats' poetry. Additionally, [3] lists several works of Keats' poetry, including an Elegy, which suggests that he was associated with the Romantic movement.

[1]-[5] do not contain specific information about what central aesthetic(s) Keats is most known for.

### q03 — Compare Whitman's free verse style in Leaves of Grass to Dickinson's compressed, slant-rhymed style — what formal element most sharply distinguishes the two?

From the provided sources:

Whitman's poem "Leaves of Grass" does not explicitly describe his free verse style. However, [1] mentions that Whitman's release date for his eBook was May 1, 1998.

Emily Dickinson's work is described in excerpt [2], where it is mentioned that her intellectual activity was great and that she had a large and characteristic choice of literary material. Excerpts [3] and [4] provide more insight into her style. According to [3], Dickinson intentionally avoided smoother and usual rhymes, and her verses show a strange cadence of inner rhythmical music. In excerpt [5], it is noted that the early editions of Dickinson's poems were edited by her friends to fit conventional forms.

A formal element that distinguishes Dickinson's style from Whitman's is not clearly present in these excerpts. However, based on the information provided:

* The structure and arrangement of lines are different between the two poets: [3] mentions that Emily Dickinson constructed lines "always daringly," whereas Whitman's poem appears to have a more free-flowing structure.
* Rhyme scheme: Dickinson's work is described as having slant-rhymes in excerpt [4], whereas there is no mention of rhymes in Whitman's poem. However, the absence of a clear rhyme scheme does not necessarily imply that Whitman did not use rhymes.

It is also mentioned in excerpt [3] that Emily Dickinson avoided smoother and usual rhymes, which could be seen as an element distinguishing her style from Whitman's, but there is no direct comparison between their styles.

### q04 — What subject matter does Whitman's Drum-Taps specifically address, and how does this differ thematically from the broader Leaves of Grass?

Based on excerpts [3] and [5], it appears that Whitman's Drum-Taps addresses themes related to war and conflict. The specific subject matter includes descriptions of soldiers arming and preparing for battle ([1]), a description of Manhattan drum-taps leading into war ([4]), and an account of the death of three soldiers in camp ([5]).

In contrast, the broader Leaves of Grass (as mentioned in [2] and hinted at in [3]'s reference to "leaves of grass") seems to address a wide range of themes, including love, democracy, individuality, nature, and transcendence. Drum-Taps appears to be more focused on specific historical events and their emotional impact on individuals.

However, the two works may share some thematic overlap, as Whitman's broader Leaves of Grass also explores ideas related to mortality ([5]) and the interconnectedness of human experience ([4]).

### q05 — Pair: Both Blake and Wordsworth wrote within visionary/Romantic nature traditions — how does Blake's symbolic, mystical approach to nature differ from Wordsworth's plain-style approach?

Based on the provided sources:

Blake's approach to nature is described as "symbolic" and "mystical", with a focus on drawing images directly or at first hand from Nature [5]. This suggests that Blake's writing style involves interpreting and representing natural phenomena in a more abstract, symbolic manner.

In contrast, Wordsworth's approach to nature is not explicitly described as mystical or symbolic. The sources mention his observation of nature, but do not provide specific details on how he represents it in his poetry. However, the text does state that Wordsworth's method of grouping his poems was "psychologically very interesting" [4], which may imply a more introspective and analytical approach to nature.

A notable difference between Blake and Wordsworth is also mentioned in [2] where William Blake's Poems of Innocence and Experience are presented, while there is no direct mention of Wordsworth's plain-style approach.

### q06 — Pair: Keats and Byron were both major Romantic poets of the same generation — how does Keats's sensuous aestheticism differ from Byron's satirical, dramatic voice?

The provided sources do not directly address a comparison between Keats's sensuous aestheticism and Byron's satirical, dramatic voice. However, there are some hints and indirect mentions that can be synthesised to provide an answer.

In [3], Arnold is quoted as saying about Byron: "Receive him absolutely without omission and compromise... follow his whole outpouring, stanza by stanza, and line by line..." This suggests that Byron's work has a dramatic quality. Additionally, in the same excerpt, it is mentioned that Byron can be "tiresome" if read entirely without selection.

In [2], there is a passage that implies Byron and Keats are being compared: "With Milton and with Keats, empowered to claim / Affinity on just one point —(or blame / Or praise my judgment, thus it fronts you full)..." However, the comparison does not explicitly describe how Keats's sensuous aestheticism differs from Byron's satirical, dramatic voice.

There are some mentions of Byron's work in [4], which includes a reference to an "edition intended to bring together all that is worthy of preservation for posterity" and notes about his relationship with Lady Caroline Lamb. However, this does not directly address the question of how Keats's sensuous aestheticism differs from Byron's satirical, dramatic voice.

[5] mentions an Epitaph for Joseph Blacket, a Poet and Shoemaker. While this may hint at the existence of other poets, it does not provide any insight into the differences between Keats's and Byron's styles.

In [1], there is no direct comparison or description of either poet's style. However, the mention of "Elegy" and "First published... 1824" suggests that Keats's work was being published around this time period.

Overall, while some indirect mentions can be made about both poets' styles, the provided sources do not explicitly compare and contrast Keats's sensuous aestheticism with Byron's satirical, dramatic voice.

### q07 — Pair: Whitman and Longfellow were both major 19th-century American poets — how does Whitman's radical free verse differ from Longfellow's formal narrative meter?

The provided sources do not contain information about Whitman's poetry. They mention Longfellow, but not in the context of his poetry style or comparison to Whitman.

Source [3] contains a passage where one poet (Longfellow) is praised alongside another (Hawthorne), but it does not describe their poetic styles or differences.

Source [5] discusses Whitman's views on European literature and culture, but it does not compare his poetry style to Longfellow's.

To answer the question accurately, I would need sources that specifically address Whitman's radical free verse and Longfellow's formal narrative meter.

### q08 — Pair: Dickinson and Poe were both American poets preoccupied with death — how does Dickinson's compressed, metaphysical style differ from Poe's Gothic, musical style?

Based on the provided sources:

Dickinson's writing style is described as "compressed" [5] and her poems feature dashes instead of traditional punctuation, except for capitalizing important words [5]. Her handwriting also undergoes a transformation from a delicate, Italian-like hand to a bolder and more abrupt one in her later years [5].

Poe's style, on the other hand, is characterized as "Gothic" and "musical" by its very nature of being entrancing and poetic, capable of evoking tears and a sense of longing for something eternal [3]. His poetry also explores the idea of struggling to apprehend the supernal Loveliness through combinations of things and thoughts of Time [3].

In terms of form, Poe's poem "The bells" is described as having multiple sections with repetitive lines about bells, while Dickinson's early poems are noted as being sparse in quantity, with her first poems written only until that winter (1862) [5].

### q09 — Pair: Rossetti and Elizabeth Barrett Browning were both major Victorian women poets — how does Rossetti's devotional, sensuous poetry differ from Browning's political, passionate poetry?

Unfortunately, the provided sources do not contain sufficient information to accurately describe the differences between Rossetti's and Browning's poetry. While [1] states that Elizabeth Barrett Browning is more read and understood in America than in England, it does not provide any details about her style or poetic themes. Similarly, [3] mentions Browning's "poetic productiveness" being affected by the death of his wife, but does not describe the nature of his poetry.

There are no sources that directly compare Rossetti's and Browning's poetry styles or themes. However, [5] does mention a conversation between Rossetti and Browning about the authorship of a poem called "Pauline", which suggests that they had some kind of literary interaction, but it does not provide any information about their poetic differences.

It is worth noting that the sources provided do not contain any explicit statements about Rossetti being a woman poet. In fact, source [5] refers to him as "Mr." and implies that he is a male poet associated with Browning's circle.

### q10 — Pair: Robert Browning and Tennyson were the two dominant male voices of Victorian poetry — how does Browning's dramatic-monologue psychological realism differ from Tennyson's elegiac, formal Poet Laureate style?

From excerpt [2], we see that Browning's conversation is "like the poetry of Chaucer, or like his own, simplified and made transparent." This suggests a focus on clarity and directness in Browning's writing. In contrast, Tennyson's poetry is described as "elegiac" and having a formal Poet Laureate style (from excerpt [5]). This implies that Tennyson's poetry tends to be more serious and formal, whereas Browning's is more straightforward.

The provided sources do not contain explicit information about the difference between Browning's dramatic-monologue psychological realism and Tennyson's elegiac, formal style. However, we can infer some differences based on general literary analysis of their works:

* Browning's dramatic monologues are characterized by a focus on the inner lives and emotions of his characters, often presented in a more realistic and psychologically nuanced way.
* Tennyson's poetry, as Poet Laureate, was expected to be more formal and reflective of the established traditions of English poetry.

However, this is not based directly on the provided sources. If we were to synthesize information from these excerpts, we could say that Browning's poetry tends to focus on individual characters' inner lives in a direct and transparent way, whereas Tennyson's poetry as Poet Laureate was expected to be more formal and traditional.

It is worth noting that the sources do not provide comprehensive biographical information about either poet.

### q11 — Bonus pair: Robert Browning and Elizabeth Barrett Browning were a real married couple — what stylistic differences distinguish their individual poetic voices despite their personal relationship?

Based on the provided sources, we can identify some stylistic differences between Robert Browning's and Elizabeth Barrett Browning's poetic voices:

* **Robert Browning**:
	+ His conversation is compared to his own poetry, suggesting a similarity in style [5].
	+ His poetry is described as being simplified and made transparent, implying a clear and direct language use [5].
	+ The Brownings lived for the most part in Italy during their married life, but Browning's poetic activity was not very prolific during this period [4].
* **Elizabeth Barrett Browning**:
	+ There are no explicit descriptions of her poetic style or voice in the provided sources.
	+ However, we can infer that she was a poet by the references to her poetry being widely read and understood in America [3] and [5].
	+ Her relationship with Robert Browning is mentioned as one of the causes for "peculiar and lasting gratitude" [4], suggesting that their personal connection might have influenced her writing.

It's essential to note that these observations are based on limited information from the provided sources. To gain a more comprehensive understanding of the stylistic differences between Robert and Elizabeth Barrett Browning, further research would be necessary.

Sources:
[3] – mentions Robert Browning's poetry being widely read in America
[4] – describes their relationship as "peculiar and lasting gratitude"
[5] – compares Robert Browning's conversation to his own poetry

### q12 — What does the "Index of Project Gutenberg Works" format (used for Tennyson and Byron in this cluster) provide, as distinct from a single poetry-collection volume?

The provided sources do not contain that information.

### q13 — Which poet in this cluster is most associated with dramatic monologue as a technique, and what does that technique formally involve?

According to the sources, Robert Browning (Source [4] and [5]) is the poet most associated with dramatic monologue as a technique. However, it's also mentioned in Source [3] that both poets are subjective and deal with objects suited to their purpose.

The formal involvement of dramatic monologue is described in Source [1]: "the objective poet... chooses to deal with the doings of men (the result of which dealing, in its pure form, when even description, as suggesting a describer, is dispensed with, is what we call dramatic poetry)" and further on: "appealing through himself to the absolute Divine mind, prefers to dwell upon those external scenic appearances which strike out most abundantly and uninterruptedly his inner light and power..."

However, it does not explicitly describe dramatic monologue.

### q14 — Cross-era: How does the plainness valued by Wordsworth's Romantic nature poetry differ from the more self-conscious irony of a Victorian poet like Tennyson or Browning?

The provided sources do not contain that information. They discuss Wordsworth's nature poetry and its relationship to his own interpretation of Nature, but they do not mention Tennyson or Browning, nor do they provide a comparison between Wordsworth's plainness and the self-conscious irony of Victorian poets.

### q15 — Near-miss: Poe and Dickinson are both American and both preoccupied with mortality — what specific formal feature would most reliably distinguish a Poe poem from a Dickinson poem even without an author byline?

Based on the provided sources, a reliable distinguishing feature between a Poe poem and a Dickinson poem is the use of punctuation. Specifically:

* Poe poems (as described in [2] and [3]) often employ traditional punctuation marks such as periods, commas, and quotation marks.
* Dickinson poems (described in [1]), particularly the later ones, discard most punctuation except for numerous dashes (-). In addition, all important words begin with capitals.

This characteristic is explicitly mentioned in source [1]: "In most of her poems, particularly the later ones,everything by way of punctuation was discarded, except numerousdashes; and all important words began with capitals."

### q16 — Near-miss: Byron and Robert Browning are both known for a theatrical poetic persona — how does Byron's Romantic-era "Byronic hero" differ from Browning's Victorian dramatic-monologue speaker?

To answer this question, let's examine the characteristics of each poet's style.

The "Byronic hero," associated with Lord Byron, is described in source [3] as follows:

* He dedicates his work to the Author of <em>Ion</em> and Sergeant Talfourd.
* His conversation and poetry are likened to those of Chaucer or his own work, being simplified and made transparent.

In contrast, Robert Browning's style is described in source [5] as follows:

* His countenance is full of vigor, freshness, and refined power, making it seem impossible that he can grow old.
* His poetry is subtle, passionate, and profound, yet he himself is simple, natural, and playful.
* He has the repose of a man who has lived much in the open air.

Browning's wife Elizabeth Barrett Browning is also described in source [5] as follows:

* She is a type of the most sensitive and delicate womanhood.

It appears that Byron's "Byronic hero" persona emphasizes a sense of intensity, power, and perhaps even brooding introspection. In contrast, Robert Browning's style seems more characterized by simplicity, naturalness, and playfulness, with his poetry being described as subtle, passionate, and profound.

However, it is worth noting that neither source [3] nor [5] explicitly discusses the differences between Byron's "Byronic hero" and Browning's dramatic-monologue speaker. The provided sources primarily focus on individual characteristics of each poet rather than comparing their styles directly.

To further explore this question, one might need to consult additional sources or examine other works by both poets to gain a more comprehensive understanding of their respective styles.

### q17 — What common Victorian-era social or religious tension does Christina Rossetti's devotional poetry often engage with?

The provided sources do not explicitly state the specific social or religious tensions that Christina Rossetti's devotional poetry engages with, but it appears to touch on themes of faith, hope, and redemption. The excerpts from [4] specifically mention Easter-Day and Christ rising, suggesting an engagement with Christian themes. However, a more comprehensive understanding of her work would require additional sources.

A more accurate answer can be derived by synthesizing information from the provided excerpts: Christina Rossetti's devotional poetry engages with the idea of hope, mercy, and redemption through the resurrection of Christ (as mentioned in [4]).

### q18 — Cross-document: Of the poets in this cluster, which two are most likely to be confused by a retrieval system due to overlapping themes of death and mortality across different eras?

After examining all 5 source excerpts, I found that the two poets who might be confused with each other due to overlapping themes of death and mortality are Emily Dickinson (mentioned in [4]) and Walt Whitman (implied by the phrase "the poet is the equable man" in [5] and mentioned in [3] as an author whose trifles were occasionally written).

However, based on the specific language and style used in each excerpt, I would argue that Emily Dickinson and Walt Whitman are not explicitly stated to be poets with overlapping themes of death and mortality. Instead, it is possible that the authors being discussed in these excerpts are actually Walt Whitman and another poet who is not explicitly mentioned.

A more plausible pair of poets who might be confused by a retrieval system due to their shared themes of death and mortality could be Emily Dickinson (mentioned in [4]) and Helen Jackson (whose letter is included in excerpt [4]). This is because both authors wrote about the themes of mortality, death, and poetry.

### q19 — Edge case: Longfellow was America's most popular 19th-century poet in his lifetime, while Whitman was initially controversial — what does this contrast suggest about the relationship between formal conventionality and contemporary popularity?

The provided sources do not directly address the question of the relationship between formal conventionality and contemporary popularity. However, we can attempt to synthesize information from the given excerpts.

Excerpt [2] discusses Whitman's views on European literature and his rejection of traditional poetic forms in favor of a more American style. It states that "even its greatest poets were not America's" and that Whitman might welcome classical poets like Juvenal, but for use rather than worship. This suggests that Whitman valued originality and American identity over adherence to traditional European literary conventions.

Excerpt [4] introduces Henry Wadsworth Longfellow as a prominent poet, but it does not provide information about his relationship with formal conventionality or contemporary popularity. 

Excerpts [1], [3], and [5] do not mention Whitman's reception in America or his contrast to other poets.

To answer the question accurately, we would need additional sources that explicitly discuss the relationship between formal conventionality and contemporary popularity, particularly concerning Longfellow and Whitman.

### q20 — What distinguishes Elizabeth Barrett Browning's "Poetical Works" thematically from the devotional focus found in Christina Rossetti's poems?

The provided sources do not contain that information.

