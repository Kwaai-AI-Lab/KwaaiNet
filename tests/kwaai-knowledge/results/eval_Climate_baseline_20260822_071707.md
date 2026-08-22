# RAG Eval Report

**KB:** `Climate`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 20 |
| Overall recall (token-overlap) | 48.4% (30.0/62) |
| Avg latency | 23164ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | According to Armstrong McKay et al., how many degrees of global warming could trigger multiple climate tipping points, and what does "tipping point" mean in this context? | 0/2 (0%) | [Graph: Context of Climate Change], Armstrong McKay et al - Exceeding 1.5C Global Warming Could Trigger Multiple Climate Tipping Points.pdf | 22163ms |
| q02 | What is CMIP6, according to Eyring et al., and what is its stated purpose in climate science? | 3/4 (75%) | [Graph: Climate Change 2014: Impacts, Adaptation, and Vulnerability], Eyring et al - Overview of CMIP6 Experimental Design and Organization.pdf | 21927ms |
| q03 | According to the IPCC AR6 Synthesis Report Summary for Policymakers, what is the overall assessment regarding human influence on the climate system? | 0/4 (0%) | Zhang et al - A Small Climate Amplifying Effect of Climate Carbon Cycle Feedback.pdf, IPCC - AR6 Synthesis Report Summary for Policymakers.pdf, Armstrong McKay et al - Exceeding 1.5C Global Warming Could Trigger Multiple Climate Tipping Points.pdf, [Graph: IPCC Sixth Assessment Report AR6], National Academies - Attribution of Extreme Weather Events in the Context of Climate Change.pdf, Eyring et al - Overview of CMIP6 Experimental Design and Organization.pdf | 19398ms |
| q04 | According to Kroeker et al., what specific effects does ocean acidification have on marine organisms? | 1/1 (100%) | [Graph: Ocean Acidification Ocean], Kroeker et al - Impacts of Ocean Acidification on Marine Organisms.pdf | 23123ms |
| q05 | What is GISTEMP, per Lenssen et al., and what does the paper's "observational uncertainty ensemble" attempt to quantify? | 1/4 (25%) | [Graph: UNCERTAINTY QUANTIFICATION Uncertainty], Lenssen et al - A NASA GISTEMPv4 Observational Uncertainty Ensemble.pdf | 18874ms |
| q06 | According to Miller et al., what region-specific sea level rise projections does the paper provide, and for which U.S. state? | 2/3 (67%) | Miller et al - Projected Sea Level Rise for Washington State.pdf, [Graph: PROJECTED RELATIVE SEA LEVEL CHANGE FOR 2100] | 17138ms |
| q07 | What does the NOAA Mauna Loa CO2 Record document, and why is Mauna Loa historically significant as a measurement site? | 0/4 (0%) | [Graph: NOAA's GlobalTemp Version], NOAA - Mauna Loa CO2 Record Documentation.pdf, Vihma - Effects of Arctic Sea Ice Decline on Weather and Climate.pdf, Zhang et al - A Small Climate Amplifying Effect of Climate Carbon Cycle Feedback.pdf, Kroeker et al - Impacts of Ocean Acidification on Marine Organisms.pdf, Eyring et al - Overview of CMIP6 Experimental Design and Organization.pdf, Lenssen et al - A NASA GISTEMPv4 Observational Uncertainty Ensemble.pdf | 21211ms |
| q08 | According to the National Academies report, what is "extreme weather event attribution," and what scientific challenge does it address? | 0/1 (0%) | National Academies - Attribution of Extreme Weather Events in the Context of Climate Change.pdf, [Graph: Presenting and Interpreting Extreme Event Attribution Studies] | 19765ms |
| q09 | According to the Ocean and Climate Platform document, what mechanism causes coral bleaching, and why is it described as an imminent threat? | 1/1 (100%) | Ocean and Climate Platform - Coral Bleaching An Imminent Threat to Marine Biodiversity.pdf, [Graph: Coral Reef Watch Coral Bleaching], Armstrong McKay et al - Exceeding 1.5C Global Warming Could Trigger Multiple Climate Tipping Points.pdf | 22080ms |
| q10 | According to Roessger et al., what is driving the seasonal increase in methane emissions from Siberian tundra? | 1/4 (25%) | Vihma - Effects of Arctic Sea Ice Decline on Weather and Climate.pdf, IPCC - AR6 Synthesis Report Summary for Policymakers.pdf, Turetsky et al - Permafrost Collapse Is Accelerating Carbon Release.pdf, [Graph: According], National Academies - Attribution of Extreme Weather Events in the Context of Climate Change.pdf, Ocean and Climate Platform - Coral Bleaching An Imminent Threat to Marine Biodiversity.pdf, Armstrong McKay et al - Exceeding 1.5C Global Warming Could Trigger Multiple Climate Tipping Points.pdf | 21819ms |
| q11 | According to Thomas and Twyman, how does climate change vulnerability intersect with social justice concerns? | 2/4 (50%) | Armstrong McKay et al - Exceeding 1.5C Global Warming Could Trigger Multiple Climate Tipping Points.pdf, IPCC - AR6 Synthesis Report Summary for Policymakers.pdf, Lenssen et al - A NASA GISTEMPv4 Observational Uncertainty Ensemble.pdf, Ocean and Climate Platform - Coral Bleaching An Imminent Threat to Marine Biodiversity.pdf, Kroeker et al - Impacts of Ocean Acidification on Marine Organisms.pdf, National Academies - Attribution of Extreme Weather Events in the Context of Climate Change.pdf, [Graph: Climate Change 2014: Impacts, Adaptation, and Vulnerability] | 21966ms |
| q12 | According to Turetsky et al., what carbon release mechanism is associated with permafrost collapse, and how does it differ from gradual permafrost thaw? | 1/1 (100%) | Armstrong McKay et al - Exceeding 1.5C Global Warming Could Trigger Multiple Climate Tipping Points.pdf, Turetsky et al - Permafrost Collapse Is Accelerating Carbon Release.pdf, IPCC - AR6 Synthesis Report Summary for Policymakers.pdf, [Graph: According] | 24269ms |
| q13 | According to Vihma, what effects does Arctic sea ice decline have on weather patterns beyond the Arctic region? | 4/4 (100%) | [Graph: Arctic Sea Ice Decline], Vihma - Effects of Arctic Sea Ice Decline on Weather and Climate.pdf | 26287ms |
| q14 | According to Zhang et al., what is the climate-carbon cycle feedback, and how significant is its amplifying effect according to the paper's title? | 0/1 (0%) | Zhang et al - A Small Climate Amplifying Effect of Climate Carbon Cycle Feedback.pdf, [Graph: Zhang et al.] | 22939ms |
| q15 | Cross-document: How does Turetsky et al.'s permafrost collapse mechanism relate to Roessger et al.'s Siberian tundra methane emissions findings — are they describing the same underlying process? | 4/4 (100%) | Vihma - Effects of Arctic Sea Ice Decline on Weather and Climate.pdf, Armstrong McKay et al - Exceeding 1.5C Global Warming Could Trigger Multiple Climate Tipping Points.pdf, Turetsky et al - Permafrost Collapse Is Accelerating Carbon Release.pdf, [Graph: TWICE THE PROBLEM Permafrost], IPCC - AR6 Synthesis Report Summary for Policymakers.pdf | 33956ms |
| q16 | Cross-document: How does Armstrong McKay et al.'s tipping-points framework relate to Zhang et al.'s carbon-cycle feedback — could a carbon-cycle feedback loop constitute or contribute to a tipping point? | 2/4 (50%) | [Graph: Zhang et al.], Armstrong McKay et al - Exceeding 1.5C Global Warming Could Trigger Multiple Climate Tipping Points.pdf, Zhang et al - A Small Climate Amplifying Effect of Climate Carbon Cycle Feedback.pdf | 29638ms |
| q17 | Cross-document: How does Kroeker et al.'s ocean acidification research relate to the coral bleaching mechanism described by the Ocean and Climate Platform — are acidification and bleaching driven by the same or different stressors? | 3/4 (75%) | Kroeker et al - Impacts of Ocean Acidification on Marine Organisms.pdf, Ocean and Climate Platform - Coral Bleaching An Imminent Threat to Marine Biodiversity.pdf, [Graph: Coral Reef Watch Coral Bleaching] | 26920ms |
| q18 | Cross-document: How does the CMIP6 model framework (Eyring et al.) relate methodologically to the IPCC AR6 Synthesis Report's conclusions? | 2/4 (50%) | IPCC - AR6 Synthesis Report Summary for Policymakers.pdf, [Graph: IPCC Sixth Assessment Report AR6], National Academies - Attribution of Extreme Weather Events in the Context of Climate Change.pdf, Eyring et al - Overview of CMIP6 Experimental Design and Organization.pdf | 30003ms |
| q19 | Near-miss: Both Lenssen et al.'s GISTEMP paper and the NOAA Mauna Loa CO2 record are long-running observational datasets — what's the key difference in what each dataset actually measures? | 0/4 (0%) | Vihma - Effects of Arctic Sea Ice Decline on Weather and Climate.pdf, NOAA - Mauna Loa CO2 Record Documentation.pdf, Kroeker et al - Impacts of Ocean Acidification on Marine Organisms.pdf, Eyring et al - Overview of CMIP6 Experimental Design and Organization.pdf, IPCC - AR6 Synthesis Report Summary for Policymakers.pdf, Lenssen et al - A NASA GISTEMPv4 Observational Uncertainty Ensemble.pdf, Zhang et al - A Small Climate Amplifying Effect of Climate Carbon Cycle Feedback.pdf, [Graph: GISTEMP Ensemble], Armstrong McKay et al - Exceeding 1.5C Global Warming Could Trigger Multiple Climate Tipping Points.pdf, National Academies - Attribution of Extreme Weather Events in the Context of Climate Change.pdf | 22963ms |
| q20 | Edge case: Thomas and Twyman's paper on climate vulnerability and social justice is the only explicitly social-science paper in an otherwise physical-science cluster — how does its inclusion change what kinds of questions this cluster can test? | 3/4 (75%) | [Graph: Climate Change 2014: Impacts, Adaptation, and Vulnerability], Vihma - Effects of Arctic Sea Ice Decline on Weather and Climate.pdf, Eyring et al - Overview of CMIP6 Experimental Design and Organization.pdf, National Academies - Attribution of Extreme Weather Events in the Context of Climate Change.pdf, IPCC - AR6 Synthesis Report Summary for Policymakers.pdf | 16857ms |

## Answers

### q01 — According to Armstrong McKay et al., how many degrees of global warming could trigger multiple climate tipping points, and what does "tipping point" mean in this context?

According to excerpt [2], exceeding 1.5°C global warming could trigger multiple climate tipping points. 

In this context, a "tipping point" is defined as a critical threshold at which a tiny perturbation can qualitatively alter the state or development of a system, as stated in excerpt [5].

### q02 — What is CMIP6, according to Eyring et al., and what is its stated purpose in climate science?

According to Eyring et al., CMIP6 is a cooperative effort across the international climate modelling and climate science communities. CMIP6 is a component of the Coupled Model Intercomparison Project (CMIP), which is organized under the auspices of the World Climate Research Programme's (WCRP) Working Group on Coupled Modelling (WGCM).

The stated purpose of CMIP6 in climate science is to address three broad questions:

1. How does the Earth system respond to forcing?
2. What are the origins and consequences of systematic model biases?
3. How can we assess future climate changes given internal climate variability, predictability, and uncertainties in scenarios?

CMIP6 aims to provide a framework for evaluating, understanding, and improving climate and Earth system models, with a focus on addressing the most pressing problems of climate variability and change. [6], [17], [20]

### q03 — According to the IPCC AR6 Synthesis Report Summary for Policymakers, what is the overall assessment regarding human influence on the climate system?

[9] The author states: "Human activities, principally through emissions of greenhouse gases, have unequivocally caused global warming, with global surface temperature reaching 1.1°C above 1850-1900 in 2011-2020."

### q04 — According to Kroeker et al., what specific effects does ocean acidification have on marine organisms?

[19] Kroeker et al. (2013) Impacts of ocean acidification on marine organisms: quantifying sensitivities and interaction with warming. Glob. Change Biol. 19:1884–96

This source does not specify the effects of ocean acidification on marine organisms.

### q05 — What is GISTEMP, per Lenssen et al., and what does the paper's "observational uncertainty ensemble" attempt to quantify?

According to the sources [3], [6], and [11], GISTEMP is a global land-ocean analysis of surface temperature anomalies. 

The "observational uncertainty ensemble" (as per source [4]) attempts to quantify the uncertainty in the GISTEMP analysis, specifically the sampling uncertainty and homogenization uncertainty in the GISTEMP global land-ocean analysis.

### q06 — According to Miller et al., what region-specific sea level rise projections does the paper provide, and for which U.S. state?

According to source [14], Miller et al. (2018) provide region-specific sea level rise projections for Washington State.

### q07 — What does the NOAA Mauna Loa CO2 Record document, and why is Mauna Loa historically significant as a measurement site?

I can't answer that.

### q08 — According to the National Academies report, what is "extreme weather event attribution," and what scientific challenge does it address?

According to the report [1]–[20], "extreme weather event attribution" refers to the scientific effort to attribute extreme weather events to climate change. The report states that the scientific issues and challenges associated with extreme event attribution vary greatly from one event type to another [4].

The report also explains that extreme event attribution relies strongly on all aspects of the understanding of extremes and their challenges [17]. It aims to provide an additional line of evidence that demonstrates the changing climate, its impacts, and consequences [10].

The scientific challenge addressed by extreme weather event attribution is to understand how climate change influences the occurrence, intensity, or frequency of extreme weather events. The report emphasizes the importance of considering various sources of uncertainty, including sampling uncertainty and station homogenization uncertainty, in order to accurately assess the role of climate change in extreme events [4].

### q09 — According to the Ocean and Climate Platform document, what mechanism causes coral bleaching, and why is it described as an imminent threat?

According to the Ocean and Climate Platform document [19], coral bleaching is caused by a rise in ocean surface temperature, which causes coral to expel its symbiotic algae, leading to coral death. It is described as an imminent threat because it can lead to massive mortality in reef ecosystems worldwide, from the Pacific to the Indian Ocean, the Caribbean, and the Red Sea, posing a significant threat to marine biodiversity.

Additionally, the document states that climate models predict a short-term extension of the El Niño phenomenon until late spring 2019, significantly increasing coral susceptibility to a new bleaching episode and massive mortality [18].

### q10 — According to Roessger et al., what is driving the seasonal increase in methane emissions from Siberian tundra?

There is no mention of Roessger et al. in the provided sources.

### q11 — According to Thomas and Twyman, how does climate change vulnerability intersect with social justice concerns?

The provided sources do not contain information from Thomas and Twyman.

### q12 — According to Turetsky et al., what carbon release mechanism is associated with permafrost collapse, and how does it differ from gradual permafrost thaw?

According to [2], Turetsky et al. warn that the sudden collapse of thawing soils in the Arctic might double the warming from greenhouse gases released from tundra. However, there is no information in [2] about the carbon release mechanism associated with permafrost collapse.

However, in [5], it is mentioned that growing thaw lakes are major sources of methane. And in [15], it is mentioned that abrupt permafrost drying at ~4°C and/or sufficiently rapid regional warming (>9°C) corresponding to ~5°C globally could act as a trigger for permafrost collapse (PFTP) driven by internal heat production in carbon-rich permafrost—“the compost bomb” instability.

It is in [19] that we find the information about the carbon release mechanism associated with permafrost collapse. It is mentioned that sudden collapse releases more carbon per square meter because it disrupts stockpiles deep in frozen layers.

According to [19], the carbon release mechanism associated with permafrost collapse is that it disrupts stockpiles deep in frozen layers, releasing more carbon per square meter compared to gradual thawing.

In comparison, gradual thawing affects the surface of frozen ground and slowly penetrates downwards [19].

### q13 — According to Vihma, what effects does Arctic sea ice decline have on weather patterns beyond the Arctic region?

According to excerpt [5], Timo Vihma presents a review on the effects of Arctic sea ice decline on weather and climate, including remote effects on weather at northern mid-latitudes. He mentions that while the results scatter on the magnitude, timing, and spatial extent of these effects, several studies suggest that the Arctic sea ice decline affects weather patterns beyond the Arctic region, including:

* Effects on the mid-latitude storm track, particularly in regions such as the Sea of Okhotsk, Hudson Bay, and the Labrador Sea (Kidson et al., 2011; Screen, 2013)
* Increased probability of circulation patterns resembling the negative phase of the North Atlantic Oscillation and Arctic Oscillation (NAO/AO)
* Increased probability of cold winters in Europe and northeastern Eurasia
* Effects on summer precipitation in Europe, the Mediterranean, and East Asia

It's worth noting that Vihma's review is based on various studies, and the effects of Arctic sea ice decline on weather patterns beyond the Arctic region are still being researched and debated.

### q14 — According to Zhang et al., what is the climate-carbon cycle feedback, and how significant is its amplifying effect according to the paper's title?

According to Zhang et al., the climate-carbon cycle feedback is a complex process involving the interactions between the climate system and the carbon cycle. The feedback is quantified as a function of two parameters: the carbon-concentration feedback parameter (β) and the carbon-climate feedback parameter (γ).

As for the significance of its amplifying effect, the paper's title states that the climate-carbon cycle feedback has a "small" amplifying effect.

### q15 — Cross-document: How does Turetsky et al.'s permafrost collapse mechanism relate to Roessger et al.'s Siberian tundra methane emissions findings — are they describing the same underlying process?

Based on the provided documents, we can analyze the information related to Turetsky et al.'s permafrost collapse mechanism and Roessger et al.'s Siberian tundra methane emissions findings.

Turetsky et al. discuss the sudden collapse of permafrost, which can lead to the release of large amounts of carbon into the atmosphere (source [19]). They also mention that the sudden collapse of thawing soils in the Arctic might double the warming from greenhouse gases released from tundra (source [19] and [20]).

Roessger et al.'s research focuses on Siberian tundra methane emissions, specifically the contribution of thawing lakes to these emissions. They present data on methane emissions from thaw lakes in Siberia, highlighting their significant role in releasing greenhouse gases into the atmosphere (source [17]).

While both studies deal with the impact of permafrost thawing on the release of greenhouse gases, they are not directly describing the same underlying process. Turetsky et al. focus on the collapse of permafrost itself, which can lead to the release of carbon and other greenhouse gases. Roessger et al., on the other hand, specifically examine the role of thawing lakes in methane emissions from the Siberian tundra.

However, there is an indirect connection between the two studies. The collapse of permafrost, as described by Turetsky et al., can lead to the formation of thaw lakes, which are a significant source of methane emissions, as reported by Roessger et al. In other words, the permafrost collapse mechanism can contribute to the formation of thaw lakes, which in turn release methane into the atmosphere.

In summary, while Turetsky et al. and Roessger et al. are investigating related topics, they are not describing the same underlying process. However, there is an indirect connection between the two studies, highlighting the complex interplay between permafrost thawing, greenhouse gas release, and the formation of thaw lakes.

References:

[17] Roessger et al. (not directly quoted in the provided excerpts, but mentioned as a source in Turetsky et al.'s work)

[19] Turetsky et al.

[20] Turetsky et al.

[19] and [20] are the same document, providing more information on Turetsky et al.'s research.

Note: Roessger et al.'s research is not directly quoted in the provided excerpts. However, it is mentioned as a source in Turetsky et al.'s work, indicating a connection between the two studies.

### q16 — Cross-document: How does Armstrong McKay et al.'s tipping-points framework relate to Zhang et al.'s carbon-cycle feedback — could a carbon-cycle feedback loop constitute or contribute to a tipping point?

[1] defines climate tipping points (CTPs) as a source of growing scientific, policy, and public concern. They occur when change in large parts of the climate system—known as tipping elements—become self-perpetuating beyond a warming threshold. [7] describes tipping points usually lead to irreversible qualitative change but reversible tipping points are possible as a special case. The authors describe such feedbacks as “threshold-free”. [18] discusses tipping elements are defined as components of the Earth system that are at least subcontinental in scale and could pass a tipping point as a result of actions this century.

[17] introduces the climate-carbon cycle feedback as one of the most important climate-amplifying feedbacks of the Earth system, and is quantified as a function of carbon-concentration feedback parameter (β) and carbon-climate feedback parameter (γ). However, the global climate-amplifying effect from this feedback loop (determined by the gain factor, g) has not been quantified from observations.

[4] discusses where systems exhibit localized tipping points (1 m to 1 km) at different forcing levels such that change does not self-perpetuate beyond a clear shared threshold, these are classed as threshold-free feedbacks because the accumulated global consequences of multiple localized tipping events remain roughly proportional to the forcing.

[17] also mentions that the climate-carbon cycle feedback is one of the most important climate-amplifying feedbacks of the Earth system, but the global climate-amplifying effect from this feedback loop (determined by the gain factor, g) has not been quantified from observations.

From these excerpts, it appears that a carbon-cycle feedback loop can be related to a tipping point, as the climate-carbon cycle feedback is one of the most important climate-amplifying feedbacks of the Earth system, and tipping points can be triggered by self-perpetuating change in tipping elements, which can be linked to the climate-carbon cycle feedback. However, the exact relationship between carbon-cycle feedback loops and tipping points is not explicitly stated in the provided excerpts.

Therefore, it can be inferred that a carbon-cycle feedback loop could contribute to or constitute a tipping point, but a more precise answer would require further analysis of the specific relationship between the two concepts.

### q17 — Cross-document: How does Kroeker et al.'s ocean acidification research relate to the coral bleaching mechanism described by the Ocean and Climate Platform — are acidification and bleaching driven by the same or different stressors?

According to [18], ocean acidification co-occurs with other aspects of climate change and human impacts on ocean systems, making it challenging to disentangle the impacts of ocean acidification from those of other stressors.

However, Kroeker et al.'s ocean acidification research, as mentioned in [17], explores the relationship between ocean acidification and the marine nitrogen cycle, but it does not directly address the coral bleaching mechanism.

The Ocean and Climate Platform's description of coral bleaching, as mentioned in [2] and [6], suggests that bleaching is primarily driven by rising ocean surface temperature, extreme weather events, and ocean acidification. The platform's description implies that ocean acidification is one of the stressors contributing to coral bleaching.

Kroeker et al.'s research does not directly address the question of whether acidification and bleaching are driven by the same or different stressors. However, it suggests that ocean acidification is a significant stressor impacting coral reefs, which is consistent with the Ocean and Climate Platform's description of coral bleaching.

In summary, while Kroeker et al.'s research focuses on the relationship between ocean acidification and the marine nitrogen cycle, it does not provide a direct answer to the question of whether acidification and bleaching are driven by the same or different stressors. However, it implies that ocean acidification is a significant contributor to the stressors driving coral bleaching, as described by the Ocean and Climate Platform.

### q18 — Cross-document: How does the CMIP6 model framework (Eyring et al.) relate methodologically to the IPCC AR6 Synthesis Report's conclusions?

The CMIP6 model framework (Eyring et al.) is related to the IPCC AR6 Synthesis Report's conclusions in the following ways:

1. **Methodological basis**: The CMIP6 model framework provides the scientific basis for the climate projections used in the IPCC AR6 Synthesis Report. The CMIP6 simulations and analysis are expected to support the IPCC Sixth Assessment Report (AR6) as well as other national and international climate assessments or special reports [18].
2. **Climate projections**: The CMIP6 model framework includes a wide range of climate models that are used to project future climate changes. These projections are used in the IPCC AR6 Synthesis Report to provide a comprehensive overview of the current state of knowledge on climate change, its impacts, and mitigation and adaptation options [16].
3. **Common standards and protocols**: The CMIP6 framework provides common standards and protocols for modeling and analysis, which enables the comparison and evaluation of different climate models and simulations. This is essential for the IPCC AR6 Synthesis Report, which aims to provide a summary of the current state of knowledge on climate change [17].
4. **Scientific rigor and peer-review**: The CMIP6 framework is designed to ensure scientific rigor and peer-review of climate model simulations and analysis. This is in line with the IPCC's commitment to producing high-quality, peer-reviewed science [17].

In terms of specific conclusions, the CMIP6 framework is likely to inform the IPCC AR6 Synthesis Report's conclusions on the following topics:

1. **Climate change projections**: The CMIP6 framework will provide the basis for the climate projections used in the IPCC AR6 Synthesis Report, including projections of temperature, precipitation, sea level rise, and other climate variables [16].
2. **Climate change impacts**: The CMIP6 framework will inform the IPCC AR6 Synthesis Report's conclusions on the impacts of climate change, including the potential effects on ecosystems, human health, and the economy [16].
3. **Climate change mitigation and adaptation**: The CMIP6 framework will provide the basis for the climate change mitigation and adaptation options presented in the IPCC AR6 Synthesis Report [16].

Overall, the CMIP6 model framework (Eyring et al.) provides the scientific basis for the climate projections and analysis used in the IPCC AR6 Synthesis Report, and is likely to inform the report's conclusions on climate change projections, impacts, and mitigation and adaptation options.

### q19 — Near-miss: Both Lenssen et al.'s GISTEMP paper and the NOAA Mauna Loa CO2 record are long-running observational datasets — what's the key difference in what each dataset actually measures?

This question cannot be answered based on the information provided in the excerpts.

### q20 — Edge case: Thomas and Twyman's paper on climate vulnerability and social justice is the only explicitly social-science paper in an otherwise physical-science cluster — how does its inclusion change what kinds of questions this cluster can test?

The provided sources do not contain information about a specific paper by Thomas and Twyman on climate vulnerability and social justice.

