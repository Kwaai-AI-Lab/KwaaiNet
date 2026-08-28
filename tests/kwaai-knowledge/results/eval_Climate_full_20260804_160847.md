# RAG Eval Report

**KB:** `Climate`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=true

## Summary

| Metric | Value |
|--------|-------|
| Questions | 20 |
| Retrieval recall (token-overlap + semantic) | 86.3% (53.5/62) |
| Generation recall (token-overlap + semantic) | 73.5% (45.6/62) |
| Scoring mode | token-overlap + semantic embedding (low=0.30, high=0.85) |
 | Avg judge score | 1.45/2.00 (20 questions scored) |
| Avg latency | 10160ms |

## Per-question results

| ID | Question | Retrieval | Generation | Judge | Sources | Latency |
|----|----------|-----------|------------|-------|---------|--------|
| q01 | According to Armstrong McKay et al., how many degrees of global warming could trigger multiple climate tipping points, and what does "tipping point" mean in this context? | 2/2 (100%) | 0.6/2 (28%) | 2/2 | [Graph: Context of Climate Change], Armstrong McKay et al - Exceeding 1.5C Global Warming Could Trigger Multiple Climate Tipping Points.pdf | 8867ms |
| q02 | What is CMIP6, according to Eyring et al., and what is its stated purpose in climate science? | 4/4 (100%) | 2.3/4 (58%) | 2/2 | [Graph: METHODS BASED ON CLIMATE AND WEATHER MODELS], Eyring et al - Overview of CMIP6 Experimental Design and Organization.pdf | 10217ms |
| q03 | According to the IPCC AR6 Synthesis Report Summary for Policymakers, what is the overall assessment regarding human influence on the climate system? | 4/4 (100%) | 3.3/4 (82%) | 2/2 | Armstrong McKay et al - Exceeding 1.5C Global Warming Could Trigger Multiple Climate Tipping Points.pdf, IPCC - AR6 Synthesis Report Summary for Policymakers.pdf, Vihma - Effects of Arctic Sea Ice Decline on Weather and Climate.pdf, National Academies - Attribution of Extreme Weather Events in the Context of Climate Change.pdf, Zhang et al - A Small Climate Amplifying Effect of Climate Carbon Cycle Feedback.pdf, Kroeker et al - Impacts of Ocean Acidification on Marine Organisms.pdf, [Graph: IPCC Sixth Assessment Report AR6] | 9791ms |
| q04 | According to Kroeker et al., what specific effects does ocean acidification have on marine organisms? | 1/1 (100%) | 1/1 (100%) | 1/2 | [Graph: Ocean Acidification Ocean], Kroeker et al - Impacts of Ocean Acidification on Marine Organisms.pdf | 8796ms |
| q05 | What is GISTEMP, per Lenssen et al., and what does the paper's "observational uncertainty ensemble" attempt to quantify? | 3.4/4 (86%) | 3.3/4 (82%) | 2/2 | [Graph: UNCERTAINTY QUANTIFICATION Uncertainty], Lenssen et al - A NASA GISTEMPv4 Observational Uncertainty Ensemble.pdf | 9728ms |
| q06 | According to Miller et al., what region-specific sea level rise projections does the paper provide, and for which U.S. state? | 3/3 (100%) | 2.2/3 (73%) | 2/2 | [Graph: ABSOLUTE SEA LEVEL PROJECTIONS Washington], Miller et al - Projected Sea Level Rise for Washington State.pdf | 9244ms |
| q07 | What does the NOAA Mauna Loa CO2 Record document, and why is Mauna Loa historically significant as a measurement site? | 4/4 (100%) | 3.3/4 (81%) | 1/2 | Eyring et al - Overview of CMIP6 Experimental Design and Organization.pdf, Lenssen et al - A NASA GISTEMPv4 Observational Uncertainty Ensemble.pdf, Kroeker et al - Impacts of Ocean Acidification on Marine Organisms.pdf, Miller et al - Projected Sea Level Rise for Washington State.pdf, Vihma - Effects of Arctic Sea Ice Decline on Weather and Climate.pdf, National Academies - Attribution of Extreme Weather Events in the Context of Climate Change.pdf, [Graph: NOAA's GlobalTemp Version], Zhang et al - A Small Climate Amplifying Effect of Climate Carbon Cycle Feedback.pdf | 13993ms |
| q08 | According to the National Academies report, what is "extreme weather event attribution," and what scientific challenge does it address? | 1/1 (100%) | 1/1 (100%) | 2/2 | National Academies - Attribution of Extreme Weather Events in the Context of Climate Change.pdf, [Graph: Presenting and Interpreting Extreme Event Attribution Studies] | 9229ms |
| q09 | According to the Ocean and Climate Platform document, what mechanism causes coral bleaching, and why is it described as an imminent threat? | 1/1 (100%) | 1/1 (100%) | 1/2 | Armstrong McKay et al - Exceeding 1.5C Global Warming Could Trigger Multiple Climate Tipping Points.pdf, [Graph: Ocean Acidification Ocean], Ocean and Climate Platform - Coral Bleaching An Imminent Threat to Marine Biodiversity.pdf | 8471ms |
| q10 | According to Roessger et al., what is driving the seasonal increase in methane emissions from Siberian tundra? | 2.7/4 (69%) | 2.5/4 (62%) | 1/2 | National Academies - Attribution of Extreme Weather Events in the Context of Climate Change.pdf, Ocean and Climate Platform - Coral Bleaching An Imminent Threat to Marine Biodiversity.pdf, IPCC - AR6 Synthesis Report Summary for Policymakers.pdf, Turetsky et al - Permafrost Collapse Is Accelerating Carbon Release.pdf, Armstrong McKay et al - Exceeding 1.5C Global Warming Could Trigger Multiple Climate Tipping Points.pdf, [Graph: According], Vihma - Effects of Arctic Sea Ice Decline on Weather and Climate.pdf | 7973ms |
| q11 | According to Thomas and Twyman, how does climate change vulnerability intersect with social justice concerns? | 1.7/4 (42%) | 2.5/4 (62%) | 2/2 | Kroeker et al - Impacts of Ocean Acidification on Marine Organisms.pdf, Armstrong McKay et al - Exceeding 1.5C Global Warming Could Trigger Multiple Climate Tipping Points.pdf, [Graph: Climate Change 2014: Impacts, Adaptation, and Vulnerability], Lenssen et al - A NASA GISTEMPv4 Observational Uncertainty Ensemble.pdf, IPCC - AR6 Synthesis Report Summary for Policymakers.pdf, Ocean and Climate Platform - Coral Bleaching An Imminent Threat to Marine Biodiversity.pdf, National Academies - Attribution of Extreme Weather Events in the Context of Climate Change.pdf | 11889ms |
| q12 | According to Turetsky et al., what carbon release mechanism is associated with permafrost collapse, and how does it differ from gradual permafrost thaw? | 1/1 (100%) | 1/1 (100%) | 1/2 | Turetsky et al - Permafrost Collapse Is Accelerating Carbon Release.pdf, IPCC - AR6 Synthesis Report Summary for Policymakers.pdf, [Graph: According], Armstrong McKay et al - Exceeding 1.5C Global Warming Could Trigger Multiple Climate Tipping Points.pdf | 8613ms |
| q13 | According to Vihma, what effects does Arctic sea ice decline have on weather patterns beyond the Arctic region? | 4/4 (100%) | 2.4/4 (59%) | 1/2 | [Graph: Arctic Sea Ice Decline], Vihma - Effects of Arctic Sea Ice Decline on Weather and Climate.pdf | 10110ms |
| q14 | According to Zhang et al., what is the climate-carbon cycle feedback, and how significant is its amplifying effect according to the paper's title? | 1/1 (100%) | 0.3/1 (28%) | 1/2 | [Graph: Zhang et al.], Zhang et al - A Small Climate Amplifying Effect of Climate Carbon Cycle Feedback.pdf | 8461ms |
| q15 | Cross-document: How does Turetsky et al.'s permafrost collapse mechanism relate to Roessger et al.'s Siberian tundra methane emissions findings — are they describing the same underlying process? | 2.8/4 (70%) | 3.3/4 (83%) | 1/2 | Vihma - Effects of Arctic Sea Ice Decline on Weather and Climate.pdf, [Graph: Permafrost Car], Armstrong McKay et al - Exceeding 1.5C Global Warming Could Trigger Multiple Climate Tipping Points.pdf, IPCC - AR6 Synthesis Report Summary for Policymakers.pdf, Turetsky et al - Permafrost Collapse Is Accelerating Carbon Release.pdf | 10497ms |
| q16 | Cross-document: How does Armstrong McKay et al.'s tipping-points framework relate to Zhang et al.'s carbon-cycle feedback — could a carbon-cycle feedback loop constitute or contribute to a tipping point? | 4/4 (100%) | 4/4 (100%) | 2/2 | [Graph: Zhang et al.], Armstrong McKay et al - Exceeding 1.5C Global Warming Could Trigger Multiple Climate Tipping Points.pdf, Zhang et al - A Small Climate Amplifying Effect of Climate Carbon Cycle Feedback.pdf | 11170ms |
| q17 | Cross-document: How does Kroeker et al.'s ocean acidification research relate to the coral bleaching mechanism described by the Ocean and Climate Platform — are acidification and bleaching driven by the same or different stressors? | 3.0/4 (74%) | 3.1/4 (78%) | 2/2 | Ocean and Climate Platform - Coral Bleaching An Imminent Threat to Marine Biodiversity.pdf, Kroeker et al - Impacts of Ocean Acidification on Marine Organisms.pdf, [Graph: Ocean Acidification Ocean] | 11556ms |
| q18 | Cross-document: How does the CMIP6 model framework (Eyring et al.) relate methodologically to the IPCC AR6 Synthesis Report's conclusions? | 3.5/4 (87%) | 3.2/4 (80%) | 2/2 | Armstrong McKay et al - Exceeding 1.5C Global Warming Could Trigger Multiple Climate Tipping Points.pdf, IPCC - AR6 Synthesis Report Summary for Policymakers.pdf, National Academies - Attribution of Extreme Weather Events in the Context of Climate Change.pdf, [Graph: IPCC Sixth Assessment Report AR6], Eyring et al - Overview of CMIP6 Experimental Design and Organization.pdf | 13125ms |
| q19 | Near-miss: Both Lenssen et al.'s GISTEMP paper and the NOAA Mauna Loa CO2 record are long-running observational datasets — what's the key difference in what each dataset actually measures? | 3.5/4 (89%) | 1.4/4 (35%) | 0/2 | Zhang et al - A Small Climate Amplifying Effect of Climate Carbon Cycle Feedback.pdf, [Graph: GHCN‐ERSST‐GISTEMP], IPCC - AR6 Synthesis Report Summary for Policymakers.pdf, Lenssen et al - A NASA GISTEMPv4 Observational Uncertainty Ensemble.pdf, Eyring et al - Overview of CMIP6 Experimental Design and Organization.pdf, Armstrong McKay et al - Exceeding 1.5C Global Warming Could Trigger Multiple Climate Tipping Points.pdf, National Academies - Attribution of Extreme Weather Events in the Context of Climate Change.pdf, Vihma - Effects of Arctic Sea Ice Decline on Weather and Climate.pdf, Kroeker et al - Impacts of Ocean Acidification on Marine Organisms.pdf | 11269ms |
| q20 | Edge case: Thomas and Twyman's paper on climate vulnerability and social justice is the only explicitly social-science paper in an otherwise physical-science cluster — how does its inclusion change what kinds of questions this cluster can test? | 2.9/4 (72%) | 4/4 (100%) | 1/2 | Eyring et al - Overview of CMIP6 Experimental Design and Organization.pdf, IPCC - AR6 Synthesis Report Summary for Policymakers.pdf, Armstrong McKay et al - Exceeding 1.5C Global Warming Could Trigger Multiple Climate Tipping Points.pdf, National Academies - Attribution of Extreme Weather Events in the Context of Climate Change.pdf, [Graph: Climate Change 2014: Impacts, Adaptation, and Vulnerability] | 10209ms |

## Answers

### q01 — According to Armstrong McKay et al., how many degrees of global warming could trigger multiple climate tipping points, and what does "tipping point" mean in this context?

[2] states that exceeding 1.5°C global warming could trigger multiple climate tipping points. 

[16] states that several tipping points may be triggered in the Paris Agreement range of 1.5 to <2°C global warming, with many more likely at the 2 to 3°C of warming expected on current policy trajectories.

A "tipping point" is defined as a critical threshold at which a tiny perturbation can qualitatively alter the state or development of a system [16].

### q02 — What is CMIP6, according to Eyring et al., and what is its stated purpose in climate science?

According to excerpt [12], CMIP6 (the sixth phase of the Coupled Model Intercomparison Project) is a more federated structure for the project. Its purpose is to "advance basic research in climate science" by addressing specific scientific questions through a hierarchy of numerical models of different complexities.

According to excerpt [16], CMIP6 will address three broad questions:

1. How does the Earth system respond to forcing?
2. What are the origins and consequences of systematic model biases?
3. How can we assess future climate change given internal climate variability, climate predictability, and uncertainties in scenarios?

Furthermore, excerpt [18] states that these questions will be addressed through the WCRP Grand Science Challenges (GCs), which include seven areas of research: understanding Earth's energy balance; improving weather and climate predictions; evaluating long-term climate change and its coastal impacts; determining how biogeochemical cycles control greenhouse gas concentrations and climate change.

Overall, CMIP6 aims to advance our understanding of the Earth system and improve climate modeling by addressing specific scientific questions through a hierarchical approach using numerical models of different complexities.

### q03 — According to the IPCC AR6 Synthesis Report Summary for Policymakers, what is the overall assessment regarding human influence on the climate system?

The IPCC AR6 Synthesis Report Summary for Policymakers states that "Human activities, principally through emissions of greenhouse gases, have unequivocally caused global warming, with global surface temperature reaching 1.1°C above 1850-1900 in 2011-2020" ([12]). This indicates a high level of confidence (high confidence) in the assessment that human influence has caused global warming.

Additionally, it is stated that "Global greenhouse gas emissions have continued to increase, with unequal historical and ongoing contributions arising from unsustainable energy use, land use and land-use change, lifestyles and patterns of consumption and production across regions, between and within countries, and among individuals" ([12]). This further emphasizes the significant role of human activities in causing climate change.

Therefore, the overall assessment is that human influence has unequivocally caused global warming and continues to contribute to increasing greenhouse gas emissions.

### q04 — According to Kroeker et al., what specific effects does ocean acidification have on marine organisms?

The provided sources do not contain the information about what specific effects ocean acidification has on marine organisms according to Kroeker et al. 

However, [19] mentions that Kroeker KJ, Kordas RL, Crim RN, and Singh GG (2010) published an article titled "Meta-analysis reveals negative yet variable effects of ocean acidification on marine organisms."

### q05 — What is GISTEMP, per Lenssen et al., and what does the paper's "observational uncertainty ensemble" attempt to quantify?

According to the provided excerpts:

* [2] mentions that GISTEMP is a global land-ocean analysis that is calculated from station records from NOAA NCEI's Global Historical Climatology Network (GHCN) monthly version 4 and Sea Surface Temperature (SST) data from NOAA's Extended Reconstructed Sea Surface Temperature version 5.
* [3] states that the GISTEMP uncertainty ensemble aims to quantify observational uncertainty in studies relying on historical temperature data.
* [10] reiterates this, stating that the paper presents a gridded GISTEMPv4 uncertainty ensemble that attempts to quantify observational uncertainty.

So, according to Lenssen et al., GISTEMP is a global land-ocean analysis of surface temperatures, and the "observational uncertainty ensemble" attempts to quantify the observational uncertainty associated with these temperature data.

### q06 — According to Miller et al., what region-specific sea level rise projections does the paper provide, and for which U.S. state?

According to [14], Miller et al.'s 2018 assessment (projectedsea.levelrise2018.pdf), the paper provides regional-specific sea level rise projections for Washington State.

In particular, it provides local estimates of vertical land motion for 171 locations along Washington's coastline and relative sea level rise projections that reflect these distributed estimates of vertical land motion.

### q07 — What does the NOAA Mauna Loa CO2 Record document, and why is Mauna Loa historically significant as a measurement site?

Unfortunately, I can't complete this task. The sources provided do not mention anything about NOAA's Mauna Loa CO2 record or its historical significance. However, based on general knowledge, the Mauna Loa Observatory in Hawaii is one of the most well-known and long-standing locations for measuring atmospheric carbon dioxide concentrations.

Mauna Loa has been continuously monitoring CO2 levels since 1958 and has become a benchmark for tracking changes in global CO2 concentrations. Its location at an elevation of over 10,000 feet (3,048 meters) above sea level makes it an ideal spot to measure atmospheric CO2 because it is far from local pollution sources and oceanic influences.

The Mauna Loa record shows that CO2 levels have been steadily increasing since the Industrial Revolution, with the largest increases occurring in recent decades. This long-term data set has provided crucial information for understanding global carbon cycles and informing climate change research.

If you'd like to know more about the specific NOAA document or its significance, please let me know and I can try to help further!

### q08 — According to the National Academies report, what is "extreme weather event attribution," and what scientific challenge does it address?

The National Academies report describes "extreme weather event attribution" as a process that estimates how the intensity or frequency of an extreme event or class of events has been altered by climate change (or by another factor, such as low-frequency natural variability) [2].

According to the report, this field addresses the scientific challenge of understanding whether specific extreme weather events are becoming more or less likely due to climate change [7]. It involves analyzing data and using statistical techniques to quantify the impact of human-induced climate change on individual extreme events.

The report emphasizes that attributing extreme weather events to climate change requires a careful consideration of various sources of uncertainty, including sampling uncertainty and station homogenization uncertainty. Additionally, it highlights the importance of addressing challenges such as defining event duration and geographic area impacted [7].

### q09 — According to the Ocean and Climate Platform document, what mechanism causes coral bleaching, and why is it described as an imminent threat?

According to [2], coral bleaching occurs when water temperatures exceed a certain threshold, causing corals to irreversibly expel their symbiotic algae, resulting in coral death. This mechanism is described as an imminent threat because it affects the world's most biodiverse ecosystems, impacting marine food webs, ocean nutrient and carbon cycling, and livelihoods of millions of people worldwide [9].

### q10 — According to Roessger et al., what is driving the seasonal increase in methane emissions from Siberian tundra?

The provided sources do not contain information about Roessger et al. nor discuss what is driving the seasonal increase in methane emissions from Siberian tundra.

### q11 — According to Thomas and Twyman, how does climate change vulnerability intersect with social justice concerns?

The provided sources do not mention authors named "Thomas" or "Twyman". They also do not discuss the intersection of climate change vulnerability with social justice concerns.

### q12 — According to Turetsky et al., what carbon release mechanism is associated with permafrost collapse, and how does it differ from gradual permafrost thaw?

The provided sources do not contain that information about Turetsky et al.'s work. However, [5] mentions a study by Merritt R. Turetsky and colleagues where they warn that the sudden collapse of thawing soils in the Arctic might double the warming from greenhouse gases released from tundra.

### q13 — According to Vihma, what effects does Arctic sea ice decline have on weather patterns beyond the Arctic region?

The provided sources do not contain a direct answer from Vihma about the specific effects of Arctic sea ice decline on weather patterns beyond the Arctic region. However, we can infer some information from other sources.

According to [1], the Arctic sea ice decline has had several remote effects, including cold-air outbreaks from the Arctic being less cold than before.

[3] mentions that it is evident that the reduction of sea ice cover has increased the heat flux from the ocean to atmosphere in autumn and early winter. This has locally increased air temperature, moisture, and cloud cover, and reduced the static stability in the lower troposphere.

[6] discusses Arctic – mid-latitude linkages, stating that perspectives of forecasting October-December weather in the marine Arctic on the basis of September sea ice conditions seem more promising.

[11] mentions that the relative increase in precipitation will be largest over the Arctic Ocean: 30 to 40% by the end of the century.

### q14 — According to Zhang et al., what is the climate-carbon cycle feedback, and how significant is its amplifying effect according to the paper's title?

According to Zhang et al. [20], the climate-carbon cycle feedback refers to a feedback loop that involves carbon-concentration feedback parameter (β) and carbon-climate feedback parameter (γ).

The provided sources do not contain specific information about the significance of its amplifying effect as per the paper's title, "A small climate-amplifying effect of climate-carbon cycle feedback".

### q15 — Cross-document: How does Turetsky et al.'s permafrost collapse mechanism relate to Roessger et al.'s Siberian tundra methane emissions findings — are they describing the same underlying process?

Based on the provided sources, there is no direct information about Roessger et al.'s paper or its findings. However, we can examine Turetsky et al.'s permafrost collapse mechanism and relate it to general knowledge in the field of permafrost research.

Turetsky et al. discuss abrupt thawing as a process that leads to sudden collapse of permafrost (cited in [5], [17], and [19]). This collapse is said to release large amounts of carbon, which can accelerate global warming.

Roessger et al.'s paper is not mentioned in the provided sources. However, based on general knowledge, I would infer that if Roessger et al. have findings about Siberian tundra methane emissions related to permafrost collapse, it might be possible for their research and Turetsky et al.'s to describe similar underlying processes.

Both studies could be examining the impact of abrupt thawing on the release of greenhouse gases from permafrost. However, without access to Roessger et al.'s specific findings, I cannot provide a definitive answer regarding their relationship to Turetsky et al.'s research.

### q16 — Cross-document: How does Armstrong McKay et al.'s tipping-points framework relate to Zhang et al.'s carbon-cycle feedback — could a carbon-cycle feedback loop constitute or contribute to a tipping point?

Armstrong McKay et al. [19] discuss climate tipping points, which occur when change in a part of the climate system becomes self-perpetuating beyond a warming threshold, leading to substantial Earth system impacts. They identify global core and regional impact tipping elements.

Zhang et al. [17], on the other hand, focuses on the climate-carbon cycle feedback loop, quantifying its strength as a function of carbon-concentration feedback parameter (β) and carbon-climate feedback parameter (γ). However, they do not explicitly address the relationship between their framework and tipping points.

However, in Zhang et al. [17], it is stated that "self-perpetuating change can also occur across non-catastrophic thresholds in unistable systems" [14]. This implies that a carbon-cycle feedback loop could contribute to a tipping point by leading to self-perpetuating change beyond a certain threshold.

In Armstrong McKay et al. [19], the authors discuss how some tipping points may be triggered at global warming levels within the Paris Agreement range (2°C). However, they also note that exceeding 1.5°C global warming could trigger multiple climate tipping points.

Therefore, it is possible that a carbon-cycle feedback loop, as described in Zhang et al. [17], could constitute or contribute to a tipping point, but this relationship is not explicitly stated and requires further investigation.

### q17 — Cross-document: How does Kroeker et al.'s ocean acidification research relate to the coral bleaching mechanism described by the Ocean and Climate Platform — are acidification and bleaching driven by the same or different stressors?

The provided sources do not contain direct information on how Kroeker et al.'s ocean acidification research relates to the coral bleaching mechanism described by the Ocean and Climate Platform. However, we can infer some connections based on the content.

Ocean acidification is mentioned in several sources as a stressor that affects corals, making them more susceptible to bleaching [3], [5]. For example, Kroeker et al.'s research (source 20) connects ocean acidification to fundamental ecological principles and conceptual models of population- and community-level effects. This suggests that ocean acidification can impact coral reefs by altering the carbonate chemistry conditions, potentially affecting coral calcification and survival.

On the other hand, the Ocean and Climate Platform's description of coral bleaching (source 6) emphasizes the role of high seawater temperatures in triggering bleaching events. The platform mentions that climate models predict a short-term extension of the El Niño phenomenon, which increases coral susceptibility to bleaching.

While ocean acidification and bleaching are related stressors, they seem to be driven by different primary causes. Ocean acidification is linked to changes in carbonate chemistry conditions, whereas bleaching is primarily triggered by high seawater temperatures. However, it's essential to note that the two phenomena can interact and exacerbate each other, making it challenging to disentangle their individual impacts (source 18).

In summary, while Kroeker et al.'s research and the Ocean and Climate Platform's description of coral bleaching provide some insights into the relationship between ocean acidification and bleaching, further research is needed to fully understand how these stressors interact and affect coral reefs.

### q18 — Cross-document: How does the CMIP6 model framework (Eyring et al.) relate methodologically to the IPCC AR6 Synthesis Report's conclusions?

The provided sources describe the CMIP6 model framework and its relation to the IPCC AR6 Synthesis Report, but they do not directly discuss how this framework relates methodologically to the report's conclusions.

However, we can infer some connections:

1. The CMIP6 experimental design and organization (Eyring et al.) is described as providing a detailed description of the DECK and CMIP6 historical simulations (Source [15]). This suggests that the CMIP6 model framework is integral to the IPCC AR6 Synthesis Report, which likely relies on these simulations for its conclusions.

2. Source [16] mentions that CMIP6 will address three broad questions: how the Earth system responds to forcing, what are the origins and consequences of systematic model biases, and how can we assess future climate changes given internal climate variability, predictability, and uncertainties in scenarios? These questions seem to be central to the IPCC AR6 Synthesis Report's conclusions about climate change.

3. The CMIP6 framework is designed to address a hierarchy of numerical models of different complexities (Source [11]). This indicates that the CMIP6 model framework allows for a range of modeling approaches, which may contribute to the diversity of scenarios and projections presented in the IPCC AR6 Synthesis Report.

4. Source [19] notes that the CMIP6 framework generates quantitative projections based on a range of assumptions, including socio-economic variables and mitigation options. These projections are not predictions or forecasts but rather scenario-based assessments (Source [20]). This methodological approach is likely reflected in the IPCC AR6 Synthesis Report's conclusions about climate change.

5. The authors of Source [15] mention that CMIP6 will continue to support the IPCC Sixth Assessment Report (AR6) as well as other national and international climate assessments or special reports. This indicates that the CMIP6 framework is designed to inform and underpin the conclusions presented in these reports, including the IPCC AR6 Synthesis Report.

While there is no direct statement about how the CMIP6 model framework relates methodologically to the IPCC AR6 Synthesis Report's conclusions, we can infer from the sources that the framework is closely tied to the report's scenarios, projections, and conclusions.

Therefore, it appears that the CMIP6 model framework underpins the IPCC AR6 Synthesis Report's conclusions by providing a robust and comprehensive set of simulations and scenario-based assessments.

### q19 — Near-miss: Both Lenssen et al.'s GISTEMP paper and the NOAA Mauna Loa CO2 record are long-running observational datasets — what's the key difference in what each dataset actually measures?

The provided sources do not contain that information.

### q20 — Edge case: Thomas and Twyman's paper on climate vulnerability and social justice is the only explicitly social-science paper in an otherwise physical-science cluster — how does its inclusion change what kinds of questions this cluster can test?

The provided sources do not contain information about Thomas and Twyman's paper on climate vulnerability and social justice. 

However, Excerpt [4] mentions a contribution from the authors to the topic of equity and social protection in the context of climate change:

Integrating climate adaptation into social protection programs, including cash transfers and public works 
programs, is highly feasible and increases resilience to climate change, especially when supported by basic services 
and infrastructure.

The greatest gains in well-being in urban areas can be achieved by prioritising access to finance to 
reduce climate risk for low-income and marginalised communities including people living in informal settlements. (high 
confidence) {4.4, 4.5.3, 4.5.5, 4.5.6}

Excerpt [16] also mentions equity and broad and meaningful participation of all relevant actors in decision making at all scales can build social trust which builds on equitable sharing of benefits and burdens of mitigation that deepen and widen support for transformative changes.

However, since Thomas and Twyman's paper is not mentioned anywhere in the provided sources, we cannot determine how its inclusion would change the types of questions this cluster can test.

