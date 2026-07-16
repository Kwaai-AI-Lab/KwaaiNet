# RAG Eval Report

**KB:** `MobyDick`  **Model:** `llama3.1:8b`

**Flags:** top_k=5  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 20 |
| Overall recall (token-overlap) | 76.9% (60.0/78) |
| Avg latency | 5615ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | What is the central plot and thematic obsession of Melville's Moby-Dick? | 3/4 (75%) | Weaver - Herman Melville Mariner and Mystic.pdf, Melville - Moby-Dick or The Whale.pdf | 27669ms |
| q02 | What is Bartleby, the Scrivener about, and how does its tone and scale differ dramatically from Moby-Dick's? | 2/4 (50%) | Melville - Bartleby the Scrivener.pdf | 10611ms |
| q03 | What is Billy Budd about, and what moral/legal dilemma does it center on? | 2/4 (50%) | Melville - Billy Budd.pdf | 4295ms |
| q04 | What is Typee, and how does it relate to Melville's own biographical experience, per Weaver's critical biography? | 2/4 (50%) | Weaver - Herman Melville Mariner and Mystic.pdf, Minnigerode - Some Personal Letters of Herman Melville and a Bibliography.pdf | 3636ms |
| q05 | What historical event does Melville's Battle-Pieces and Aspects of the War address, and how does this differ subject-wise from his sea narratives? | 4/4 (100%) | Melville - Battle-Pieces and Aspects of the War.pdf, Minnigerode - Some Personal Letters of Herman Melville and a Bibliography.pdf | 3727ms |
| q06 | What real 1820 historical event does the whaling-industry literature in this cluster provide context for, that directly inspired Moby-Dick's plot? | 2/4 (50%) | Melville - Moby-Dick or The Whale.pdf, Weaver - Herman Melville Mariner and Mystic.pdf, Melville - Billy Budd.pdf | 3270ms |
| q07 | What does the Book of Job (KJV) concern, and what thematic parallels does it have with Moby-Dick, such as confrontation with an unknowable natural force? | 3/4 (75%) | Weaver - Herman Melville Mariner and Mystic.pdf, King James Version - The Book of Job.pdf | 6397ms |
| q08 | According to Weaver's "Herman Melville: Mariner and Mystic," what dual character does the biography's title suggest about Melville? | 3/3 (100%) | Weaver - Herman Melville Mariner and Mystic.pdf, Minnigerode - Some Personal Letters of Herman Melville and a Bibliography.pdf | 2196ms |
| q09 | What does D.H. Lawrence's Studies in Classic American Literature argue about Moby-Dick, based on its inclusion as literary criticism in this cluster? | 4/4 (100%) | Weaver - Herman Melville Mariner and Mystic.pdf, Melville - Moby-Dick or The Whale.pdf | 2971ms |
| q10 | What does Browne's Etchings of a Whaling Cruise document, and how does its firsthand account compare to Melville's fictionalized depiction in Moby-Dick? | 3/4 (75%) | Melville - Moby-Dick or The Whale.pdf, Weaver - Herman Melville Mariner and Mystic.pdf, Browne - Etchings of a Whaling Cruise.pdf | 3558ms |
| q11 | What does Minnigerode's work compile regarding Melville, and what two types of material does its title indicate? | 4/4 (100%) | Melville - Typee.pdf, Minnigerode - Some Personal Letters of Herman Melville and a Bibliography.pdf | 2567ms |
| q12 | To whom did Melville dedicate Moby-Dick, and what work by that author is included in this cluster as contemporary context? | 4/4 (100%) | Minnigerode - Some Personal Letters of Herman Melville and a Bibliography.pdf, Weaver - Herman Melville Mariner and Mystic.pdf | 3965ms |
| q13 | Cross-document: How does Hawthorne's The Scarlet Letter compare stylistically and thematically to Moby-Dick, given the two authors were contemporaries and friends? | 4/4 (100%) | Weaver - Herman Melville Mariner and Mystic.pdf, Hawthorne - The Scarlet Letter.pdf | 4952ms |
| q14 | Cross-document: How does Starbuck's History of the American Whale Fishery provide factual and industrial context that Melville draws on for Moby-Dick's whaling-industry digressions? | 2/4 (50%) | Starbuck - History of the American Whale Fishery.pdf, Weaver - Herman Melville Mariner and Mystic.pdf, Melville - Moby-Dick or The Whale.pdf | 5896ms |
| q15 | Near-miss: Both Typee and Moby-Dick are sea narratives by Melville, but Typee is largely autobiographical while Moby-Dick is fully fictionalized — what does this distinction suggest about Melville's development as a writer? | 4/4 (100%) | Weaver - Herman Melville Mariner and Mystic.pdf, Melville - Moby-Dick or The Whale.pdf | 5826ms |
| q16 | Near-miss: Both Bartleby and Billy Budd deal with authority and refusal, but in an office setting versus a naval setting respectively — how does each setting shape its exploration of institutional power? | 4/4 (100%) | Melville - Billy Budd.pdf | 5096ms |
| q17 | Cross-document: How might Weaver's critical biography and Minnigerode's letters/bibliography compilation serve different scholarly purposes for someone studying Melville? | 3/4 (75%) | Minnigerode - Some Personal Letters of Herman Melville and a Bibliography.pdf | 5570ms |
| q18 | Cross-document: What thematic throughline connects the Book of Job's confrontation with an overwhelming natural force to Ahab's confrontation with the white whale in Moby-Dick? | 3/4 (75%) | Melville - Moby-Dick or The Whale.pdf | 3271ms |
| q19 | Edge case: Which single document in this cluster would be hardest to distinguish from Moby-Dick using only thematic or topical similarity, rather than direct textual overlap? | 2/4 (50%) | Weaver - Herman Melville Mariner and Mystic.pdf, Minnigerode - Some Personal Letters of Herman Melville and a Bibliography.pdf, Melville - Moby-Dick or The Whale.pdf | 3866ms |
| q20 | What does the presence of both fiction (Melville's other novels) and non-fiction (Starbuck's history, Browne's memoir) in this cluster allow a retrieval system to be tested on, that a fiction-only cluster could not? | 2/3 (67%) | Melville - Moby-Dick or The Whale.pdf, Weaver - Herman Melville Mariner and Mystic.pdf, Minnigerode - Some Personal Letters of Herman Melville and a Bibliography.pdf | 2980ms |

## Answers

### q01 — What is the central plot and thematic obsession of Melville's Moby-Dick?

According to the sources, the central plot and thematic obsession of Melville's Moby-Dick can be summarized as follows:

* The central plot revolves around the hunt by Captain Ahab after the great white whale Moby Dick [3].
* The organizing theme of this unparalleled volume is the hunt by the mad Captain Ahab after the great white whale [2].
* This book, Moby-Dick, is a masterpiece that has been called "one of the world's great works of art" [1].
* Melville seems to have spoken the very secret of the sea and drawn into his tale all the magic, sadness, and wild joy of many waters [2].

The thematic obsession appears to be:

* Revenge: Captain Ahab is driven by a monomaniacal desire for revenge against Moby Dick, which had dismasted him in the past [5].
* The self-sustaining vitality of nature: Melville has enforced his moral by giving an independent and ideal verisimilitude to its innocent and unconscious exponents, much like Bunyan or Swift [3].
* The blurred line between good and evil: Heaven itself seems to step in to take out of Ahab's hands the damning thing he would have done, suggesting that there is a larger force at work beyond human morality [4].

Overall, Moby-Dick is a complex novel that explores themes of obsession, revenge, nature, and morality through its epic story of Captain Ahab's hunt for the white whale.

### q02 — What is Bartleby, the Scrivener about, and how does its tone and scale differ dramatically from Moby-Dick's?

I cannot answer this question as it involves multiple sources. 

However, I can try to help you analyze the excerpts provided.

From the provided excerpts, we see that "Bartleby, the Scrivener" appears to be a short story about an attorney who employs a scrivener (law-copyist) named Bartleby. The narrator is drawn into a series of observations and reflections on his relationship with Bartleby, which becomes increasingly complex and unsettling.

As for how its tone and scale differ dramatically from Moby-Dick's, the excerpts don't provide enough information to make a direct comparison. However, it seems that "Bartleby" has a more introspective and philosophical tone, whereas Moby-Dick is known for its epic scope, symbolism, and action-packed narrative.

To answer your question fully would require reading the entire stories or having external knowledge about them.

### q03 — What is Billy Budd about, and what moral/legal dilemma does it center on?

The provided sources do not contain a clear description of what "Billy Budd" is about. However, we can infer from [1] that the story follows the character Billy Budd, who is scheduled to be executed at dawn.

As for the moral/legal dilemma, [4] describes it as a "moral phenomenon presented in Billy Budd." According to [2], the issue revolves around Billy's innocence and his potential involvement in plotting disaffection aboard the ship. However, [1] presents a more nuanced view of Billy, depicting him as someone who is aware of death but does not understand its significance.

The core moral/legal dilemma seems to be centered on the character Claggart, who accuses Billy of plotting against the ship's authority ([5]). Billy Budd asks Claggart for evidence, which leads to an intense confrontation between the two characters. This sets up a tension between Billy's innocence and the perceived threat he poses to the ship's order.

[3] mentions that "Billy Budd" is part of The Electronic Classics Series, but it does not provide any information about the story's plot or moral/legal dilemma.

Overall, the sources suggest that "Billy Budd" explores themes of morality, innocence, and the complexities of human nature.

### q04 — What is Typee, and how does it relate to Melville's own biographical experience, per Weaver's critical biography?

According to source [5], Typee contains an account of Melville's own experiences in the Marquesas Islands during a voyage on which he shipped before the mast on the whaler Acushnet. This implies that Typee is a semi-autobiographical work, drawing from Melville's actual experiences.

Additionally, source [5] mentions that up to January 1849, 6392 copies of Typee had been sold in America, netting Melville $655.91; and in England, up to April 29, 1851, 7437 copies, netting $708.40. This suggests that Typee was a commercially successful book for Melville.

The provided sources do not contain specific information about the relationship between Melville's experiences and the events depicted in Typee.

### q05 — What historical event does Melville's Battle-Pieces and Aspects of the War address, and how does this differ subject-wise from his sea narratives?

Melville's Battle-Pieces and Aspects of the War addresses the American Civil War (1861-1865). This differs subject-wise from his sea narratives in that it focuses on land battles and politics rather than maritime adventures. 

In contrast to his sea narratives like Moby-Dick, which explores themes related to the sea, nature, and human existence, Battle-Pieces deals with the war's impact on American society, patriotism, and morality. The collection is a response to the Civil War, and it examines various aspects of the conflict through poems that touch on battles, military strategies, and the emotional toll of the war on individuals and communities.

In particular, Melville critiques the conduct of the war, reflecting on its moral implications and questioning the motivations behind the conflict. He also acknowledges the bravery and sacrifice of soldiers from both sides, highlighting the complexities and nuances of the war experience.

### q06 — What real 1820 historical event does the whaling-industry literature in this cluster provide context for, that directly inspired Moby-Dick's plot?

The whaling-industry literature in this cluster provides context for the sinking of the Essex, a Nantucket whale ship, on November 20, 1820. This event directly inspired Melville to recount it in Moby-Dick [5].

This event is specifically mentioned as being recounted in Moby-Dick and also described in Owen Chase's Narrative of the Most Extraordinary and Distressing Ship Wreck of the Whale Ship Essex of Nantucket, published in 1821, which was drawn upon by Melville in writing Moby-Dick [5]. 

Therefore, the answer is that the literature provides context for the sinking of the Essex on November 20, 1820.

### q07 — What does the Book of Job (KJV) concern, and what thematic parallels does it have with Moby-Dick, such as confrontation with an unknowable natural force?

The Book of Job (KJV) concerns the story of Job, a righteous man who suffers greatly due to circumstances beyond his control. He loses his children, health, and wealth, prompting his friends Eliphaz, Bildad, and Zophar to offer explanations for his sufferings, which ultimately leads to a confrontation with God.

According to source [4], Moby-Dick can also be seen as a book that explores the theme of confrontation with an unknowable natural force. Ahab, the protagonist, is obsessed with hunting down the white whale Moby-Dick, which he sees as a symbol of his own existential struggle against nature. Melville describes Ahab's crew as being "chiefly made of mongrel renegades, and cast-aways and cannibals" [4], suggesting that they are also grappling with the unforgiving power of nature.

Both Job and Ahab experience immense personal loss and suffering, leading to a crisis of faith in the face of an unknowable natural force. In both stories, there is also a sense of moral ambiguity, as the characters struggle to understand the motivations behind their suffering. While Job's friends offer explanations for his sufferings that are ultimately proven wrong, Ahab's quest for Moby-Dick can be seen as a futile attempt to impose human will on an indifferent natural world.

Source [4] explicitly mentions that "Ahab is, of course, the atheistical captain of the tormented soul" and that he is "morally en-feebled... by the incompetence of mere unaided virtue or rightmindedness in Starbuck", suggesting a deeper exploration of the human condition in Moby-Dick. In contrast, the Book of Job (KJV) presents a more straightforward narrative about a righteous man's struggle with suffering and God's eventual response.

The thematic parallels between the two works suggest that both authors are exploring similar ideas about the human relationship with nature, morality, and the search for meaning in the face of uncertainty. However, the tone and style of the two works differ significantly, reflecting the distinct literary and cultural contexts in which they were written.

### q08 — According to Weaver's "Herman Melville: Mariner and Mystic," what dual character does the biography's title suggest about Melville?

The title "Mariner and Mystic" suggests that Herman Melville had a dual character as both a sailor (mariner) and a person with spiritual or mystical inclinations. [1]

### q09 — What does D.H. Lawrence's Studies in Classic American Literature argue about Moby-Dick, based on its inclusion as literary criticism in this cluster?

The provided sources do not contain any information about D.H. Lawrence's Studies in Classic American Literature or what it argues about Moby-Dick.

However, excerpt [5] mentions "the elementary force of Moby-Dick, its vitality, its thrilling power" as an agreement among critics, but does not specifically mention D.H. Lawrence's work. Excerpt [2] also mentions a critic (E.L. Grant Watson) who praises Melville, but there is no indication that it is referencing D.H. Lawrence.

### q10 — What does Browne's Etchings of a Whaling Cruise document, and how does its firsthand account compare to Melville's fictionalized depiction in Moby-Dick?

According to [1] and [2], J. Ross Browne's Etchings of a Whaling Cruise documents the life of a whaleman with a focus on factual accuracy, aiming "at the truth itself than at mere polish" ([2]). The excerpt states that Browne provides a faithful delineation of the life of a whaleman.

In contrast to Browne's approach, Melville's Moby-Dick is described in [1] and [2] as glorifying the romance of whales, with a focus on rhetoric and characterization rather than straightforward documentation. Specifically, [1] notes that Browne provokes no "rhetorical extravagance of characterisation," while Melville uses such techniques to describe scenes like the one involving Haggard and Ahab ([3]).

### q11 — What does Minnigerode's work compile regarding Melville, and what two types of material does its title indicate?

Minnigerode's work, titled "Some Personal Letters of Herman Melville and a Bibliography", compiles personal letters of Herman Melville. Its title indicates that the work contains two types of material: personal letters (of Melville) and a bibliography.

[1] mentions Minnigerode's work as compiling "Some Personal Letters" along with "a bibliography" [M56].

### q12 — To whom did Melville dedicate Moby-Dick, and what work by that author is included in this cluster as contemporary context?

According to [1], Melville dedicated Moby-Dick to Nathaniel Hawthorne. 

In the same excerpt, a book by Nathaniel Hawthorne, "The Wonder Book", is mentioned, but not specifically as being part of the contemporary context. However, it provides some information about Hawthorne's work.

Excerpt [3] includes information about a letter from Hawthorne to Melville and how Melville dedicated Moby-Dick to him. It does not provide direct evidence that Hawthorne's works are part of the contemporary cluster but rather contextualizes their relationship through personal correspondence.

[5] provides a quote from Hawthorne's "The Wonder Book", which is one of his published works, mentioning Herman Melville and his conception of Moby-Dick.

Therefore, to answer your question, Melville dedicated Moby-Dick to Nathaniel Hawthorne. The contemporary context for this event includes Hawthorne's work, specifically his novel (implied by the mention in [5]), but not explicitly named.

### q13 — Cross-document: How does Hawthorne's The Scarlet Letter compare stylistically and thematically to Moby-Dick, given the two authors were contemporaries and friends?

The provided sources do not contain explicit statements about Hawthorne's The Scarlet Letter being compared stylistically or thematically to Melville's Moby-Dick. However, based on excerpts [4] and [5], we can infer some connections between the two works:

* Excerpt [4] mentions Herman Melville shaping out the conception of his white whale in Pittsfield, while Hawthorne is implied to be working on The Scarlet Letter around that time.
* Excerpt [5] suggests a connection between Hawthorne's style and Melville's writing, as it states: "When Melville speaks of 'the calm, the coolness, the silent grass-growing mood in which a man ought to compose,' he has caught a demoralisation from Hawthorne." This implies that Melville was influenced by Hawthorne's style.

There is no explicit thematic comparison between the two works. However, we can note that both authors deal with themes of morality, sin, and redemption:

* The Scarlet Letter explores the theme of sin and guilt in Puritan society.
* Moby-Dick explores the theme of obsession and the nature of man versus the sea.

It is also worth noting that Hawthorne's Preface to The Custom-House mentions an autobiographical impulse, which could be seen as a stylistic connection between the two authors' works.

A more comprehensive analysis would require additional information about the authors' relationships, influences, and writing processes.

### q14 — Cross-document: How does Starbuck's History of the American Whale Fishery provide factual and industrial context that Melville draws on for Moby-Dick's whaling-industry digressions?

Based on the provided sources, I found some relevant information to answer your question.

**Historical context**: [1] mentions "the literature of whaling slopes down from Moby-Dick, both before and after, into a wilderness of several hundred volumes." This implies that Melville's novel is part of a larger literary landscape of whaling-themed works. [3] provides some context on the Pequod's journey: Ahab intends to pass through certain straits into the Javan sea, where Moby Dick might be found.

**Whaling-industry specifics**: Walter S. Tower's A History of the American Whale Fishery (Philadelphia, 1907) is mentioned in [1] and provides factual information on whaling. However, only a snippet of this text appears to be relevant: "sailing from American ports—Continued. / Brazil / Date— Result of voyage" in [5].

**Relationship between Melville's novel and the industry**: There isn't explicit information stating that Melville draws on Tower's History for Moby-Dick's whaling-industry digressions. However, the mention of Tower's work alongside Moby-Dick suggests a connection between literary and historical accounts.

To provide more comprehensive insight into Starbuck's History and its relation to Melville's novel would require access to the full text of Tower's book, which is not provided here.

Considering these points, I can say that:

* The sources hint at the existence of extensive whaling literature before Moby-Dick.
* Some specific historical context regarding Ahab's plan is mentioned in [3].
* There might be a connection between Melville's work and Tower's History, but without access to the full text, this relationship remains unclear.

Please let me know if you'd like me to clarify any aspect of my response!

### q15 — Near-miss: Both Typee and Moby-Dick are sea narratives by Melville, but Typee is largely autobiographical while Moby-Dick is fully fictionalized — what does this distinction suggest about Melville's development as a writer?

Based on the provided sources [1]–[5], we can note that:

* Melville's "Typee" was considered by Arthur Stedman in 1892 to be one of Melville's highest achievements, suggesting that it was highly regarded for its autobiographical elements and novelty at the time. (Source: [3])
* In contrast, Moby-Dick is described as an allegory designed to teach "woeful wisdom" with a "gay science" of pessimism. This suggests that Melville aimed for a more philosophical or symbolic exploration in his later work, rather than relying solely on personal experience. (Sources: [1] and [2])
* The popularity of Moby-Dick's style and the number of imitators it spawned imply that Melville had developed a distinct narrative voice and approach by this time, which was emulated by others. (Source: [3])

This distinction suggests that Melville's development as a writer involved:

1. Expanding his thematic scope from personal experience to more philosophical or symbolic explorations.
2. Developing a unique narrative voice and style that could be emulated by others.

However, the sources do not provide information on whether this transition was a deliberate choice or an organic evolution in Melville's writing process.

Additional observations:

* The fact that Melville's contemporaries recognized and praised Typee as an autobiographical masterpiece indicates that his personal experiences were seen as a valuable contribution to literature.
* The comparison between the two works also highlights the importance of reader reception and criticism, as Stedman's praise for Typee likely influenced how readers approached Moby-Dick.

The provided sources do not contain explicit statements about Melville's development as a writer or his intentions behind this shift.

### q16 — Near-miss: Both Bartleby and Billy Budd deal with authority and refusal, but in an office setting versus a naval setting respectively — how does each setting shape its exploration of institutional power?

To address this question, let's examine the provided sources.

Billy Budd is set on a naval ship during war time [3]. The novel explores the relationship between Billy, who is perceived as being in the "King's bargain," and the authority figures on board, including Captain Vere. The setting of the naval ship allows for an examination of institutional power within a rigid hierarchical structure, where discipline and obedience are paramount.

In contrast, Bartleby, another novel by Herman Melville, deals with office politics [not provided in the excerpts]. A common thread between the two works is the theme of authority and refusal. However, to fully address your question, we would need access to the specific information about Bartleby's setting and exploration of institutional power.

Since we don't have direct access to the information about Bartleby in these excerpts, I can only speculate that the office setting might shape its exploration of institutional power by focusing on bureaucratic red tape, corporate culture, and the effects of authority within a more sedentary environment. However, this is purely speculative without further context.

To provide a more comprehensive answer based on the provided sources:

* The naval setting in Billy Budd shapes its exploration of institutional power by highlighting the rigid hierarchy, discipline, and obedience required in such an environment [3].
* We lack direct information about Bartleby's setting and exploration of institutional power, but we can speculate that it might focus on bureaucratic or corporate culture.

If you have access to further sources or context about Bartleby, I would be happy to provide a more comprehensive answer.

### q17 — Cross-document: How might Weaver's critical biography and Minnigerode's letters/bibliography compilation serve different scholarly purposes for someone studying Melville?

Based on the provided sources, here are some possible ways in which Weaver's critical biography and Minnigerode's letters/bibliography compilation might serve different scholarly purposes:

**Weaver's Critical Biography:**

* Provides a more in-depth analysis of Melville's life and works, with a focus on his literary development, themes, and style [4].
* Offers insights into Melville's relationships with other notable authors, such as Hawthorne [4].
* May include critical evaluations of specific works, such as "Pierre," which is mentioned as containing autobiographical material [3].
* Could be used to understand the intellectual and literary context in which Melville wrote, including his influences and contemporaries.
* Is likely written with a more contemporary critical perspective, reflecting the author's own scholarly interests and biases.

**Minnigerode's Letters/Bibliography Compilation:**

* Offers a primary source collection of letters from Melville himself, providing unique insights into his thoughts and experiences [1].
* Includes facsimiles of actual letters from Melville to Evert Duyckinck, offering tangible evidence of his interactions with other writers and individuals [2].
* Serves as a comprehensive bibliography of Melville's works, listing numerous titles and editions [5].
* Can be used to trace the publication history of Melville's books and their reception by readers and critics over time.
* May provide valuable information for literary historians interested in the publishing industry and book culture during the 19th century.

In summary, Weaver's critical biography might be useful for scholars seeking a more nuanced understanding of Melville's life and works, while Minnigerode's letters/bibliography compilation could serve as a vital resource for researchers examining primary sources, publication history, and literary context.

### q18 — Cross-document: What thematic throughline connects the Book of Job's confrontation with an overwhelming natural force to Ahab's confrontation with the white whale in Moby-Dick?

The thematic throughlines connecting the two works are:

Ahab and the White Whale can be seen as a modern-day equivalent of God and Satan, or the Devil.

In both cases, there is an "overwhelming natural force" - in Job's case, it is God's power and wrath; in Ahab's case, it is Moby-Dick, the white whale [3] [4].

Ahab can be seen as a character struggling with his own "hubris and pride", much like Job who is faced with his own limitations and insignificance in comparison to God's majesty.

### q19 — Edge case: Which single document in this cluster would be hardest to distinguish from Moby-Dick using only thematic or topical similarity, rather than direct textual overlap?

Based on the provided excerpts, I would say that [4] is the hardest to distinguish from Moby-Dick using only thematic or topical similarity. 

The excerpt lists a series of book titles and page numbers, which may suggest a connection to literature or writing, but it doesn't provide any direct textual overlap with Moby-Dick. However, since Melville is listed as an author in the same collection as Moby-Dick (alongside other works such as Redburn, The Piazza Tales, etc.), this excerpt can be seen as connected to Moby-Dick through its shared authorship.

Other excerpts either provide direct textual overlap with Moby-Dick ([1], [2]), or discuss specific topics related to the book (e.g. Ahab, the Shaker God, etc.). [5] is an essay discussing Moby-Dick's structure and writing style.

### q20 — What does the presence of both fiction (Melville's other novels) and non-fiction (Starbuck's history, Browne's memoir) in this cluster allow a retrieval system to be tested on, that a fiction-only cluster could not?

The presence of both fiction and non-fiction works in this cluster allows the retrieval system to be tested on its ability to distinguish between different genres or types of documents. A fiction-only cluster would only test the system's performance within one genre, but with a mix of both fiction and non-fiction, the system must also demonstrate its ability to identify features that are unique to each type, such as the use of rhetorical extravagance in Melville's writing versus the more straightforward and factual approach of Browne's memoir.

