# Early Thoughts — Keep Notes (2019-2020)
**Source**: Personal notes from Keep app
**Relevance**: Earliest recorded ideas about language design and lambda calculus applications

---

## Note 1: JS like typeless language design
**Date**: May 22, 2019

Today I'm thinking about a number of things. First time thinking about creating my own programming language. Second I'm thinking about how to Leverage open source libraries to write a machine learning algorithm that assist with development, whether that be code formatting, fixing packages versions for security purposes, or just code autocomplete. In relation to the first thing of creating my own programming language oh, there was a man I met name Kieran Brown who spoke of a typeless language. While thinking about this I've realized there is serious opportunity in this. In JavaScript there has been a great accomplishment and advancement in the field of software development as a result of their design philosophy. JavaScript has simplify many of the concepts commonly thought of in programming down to the Core Essence. In JavaScript you do not have the idea of classes you have the idea of maps or JavaScript objects. JavaScript you do not have integers long numbers big integers Etc, but instead you have this idea of a number. In JavaScript you do not have many different types of lists you interact with, but instead have one type of list that you iterate through, generally speaking. Don't get me wrong, JavaScript has many flaws and unnecessary complexities, but one thing is true for searching about JavaScript that through their design philosophy of development they have made the lives of developers immensely better. Earlier today I was thinking about a type of language that might use a similar design philosophy that makes JavaScript great but even better. What if there was a language where there was no distinguishing between Primitives but instead there was only iterables of characters. One way to think about this for example is thinking of all types as strings. My immediate thought when this idea popped in my head is what about efficiency? My second thought was well I think a language should care about efficiency where is developer should care about getting their ideas into a computer. One can always optimize a language based on context more. You can add an Edge case in the compiler or in the runtime and so on. A developer should not be thinking all the time about efficiency. Languages can do that for them. Developers should be concerned with creating applications and languages need to get out of there way more. The argument with pipes is that types do not get in the way but allow granularity of specificity. I would challenge this notion regardless of intent to increase specificity poor developers will always mess it up. No matter what type system you have Developers often do things in Waze that produce bugs. There's no way around this but what if our language was to get out of your way and you had standard language practices that prevented poor use, this is the debate between correctness and flexibility. If you haven't been able to tell so far I am currently on the side of flexibility. I want to communicate to a computer the way that I think not the way the computer thinks. I want my language to care about how a computer thinks and behind-the-scenes translate my thoughts into the computer thoughts, not the other way around. So how would want to complish this typeless language I am speaking of? Well currently my thought is as I said before everything basically being an iterable characters. This solved the issue of all types. All Primitives are then strings, and all lists and Maps or JavaScript objects become strings as well through the iterable structure being nested nestable. I still think functions might be required though. At this moment I haven't figured out how to make a function into an iterable of characters or vice versa. If I was able to make inexorable of characters from a function, maybe everything would be a function instead. Actually now that I am thinking about this out loud, this is actually possible. It is possible to create an editable of values purely with functions. Think of a linked list function using the parameters to store state. In this case you could simplify your typing system down to two things functions and characters. One concern that does come to mind is what if someone was to create is a normal typed language using this language. One could overcome these this challenge through the benevolent dictator owner of a language idea, like in Elm and so on. Competing thought is what I really even want to prevent use of the language in such a way. In my mind I probably wouldn't. The reason being that if someone wants to convert a good language into a bad language, for example JavaScript into Scala, they can do this. Just because they are not leveraging JavaScript and it's Creative Design philosophy, doesn't mean that JavaScript is no longer a great programming language. Syntax is obviously miles down the road, but one thing to also consider when creating my own language that is quote-unquote typeless is ease-of-use for beginners. If it is helpful to think of something as a map or as a list maybe the high-level construct should be displaying it as such. Also so efficiencies in a language is syntax are desirable the language is syntax should be optimized for beginner readability and not 4 design efficiency initially. In my mind this is actually all accomplishable, and part of me wonders if it already exists. Later on I'll be doing searching for a typeless functional programming language. Lambda calculus might also be a way of reducing the complexity from two language constructs down to one language construct. You can represent boolean's using a function and you can represent strings with a function oh, and you can represent numbers with a function. A important question to ask myself is whether or not reducing things down to their Core Essence should bubble up to the language user. It might be interesting to develop the language core in such a way where everything is using Lambda calculus, but bubble-up syntax to the user that is minimizing complexity in maximizing developer programming efficiency. There might not be a right answer, but I believe there to be most definitely opportunity there. Something else that I am thinking about in relation to a typeless language, is the idea of helpful separation between Concepts. Part of me feels like the separation for example between objects and arrays in JavaScript might help developers learn the mindset required to be a good developer. Through thinking of everything as either a map or an array of Primitives programmers are able to truly reduce programming down to its Core Concepts. Though in the end programming language might have more constructs than what is necessary, those extra constructs might be serving a purpose to reduce actual complexity When developing. A possible alternative to a Typeless language, or just a better alternative to JavaScript using a similar design philosphy, might be this idea of JavaScript with pythonic/ Haskell like syntax

---

## Note 2: Things to write about
**Date**: Unknown

- [ ] I love my life more than I love myself
- [ ] A drag and drop app where you can build website, that also allows you to use voice to change things
- [ ] A voice app coworker, with a 2 way Alexa that will just randomly talk to you, ask you questions about your work, you can setup reminders and scripts for it to run
- [ ] How combinatorics creates varying increasingly more complex variations of possibilities over time, but we still have a predictable world
- [ ] An Examined Life: Mindfulness and it's Results
- [ ] Ratio of wanting to liking is the heartbeat of addiction
- [ ] Mental health, and living a happy life can be measured by the time it takes for your emotionals to reach baseline, similar for heart rate to baseline for physical fitness is a good metric
- [ ] Competency is lesser than relationships in a work setting
- [ ] Shared fiction frame of reference is why games work
- [ ] Expectations framework
- [ ] Obligatory Metaethics
- [ ] My epistemology
- [ ] Modeling the world
- [ ] What makes good romantic relationships/marriage
- [ ] Nature of Marriage
- [ ] What makes a good life
- [ ] How I would ideally problem solve in marriage
- [ ] Stages of romantic relationships and marriage
- [ ] Relationships Heuristics
- [ ] Ideal woman
- [ ] - Future of Networked AR displays
- [ ] - mini retirements
- [ ] - music is about tension and release, or a balance between caos and order
- [ ] - distributed operating systems
- [ ] - Implicit multicore processing
- [ ] - lambda calculus based hardware
- [ ] - lambda calcs connection to how neurons work
- [ ] - analog computing
- [ ] Getting rid of the screen door effect in VR with staggered pixels
- [ ] Summarization AI
- [ ] 90% or something of the english language is reduntant.
- [ ] What if you were to create music where you made people think you were going to say one thing and then say a gibberish word that sounds similar, thus creating tension and release
- [ ] Flash light into peoples eyes with different colors in different frequencies to change mood, maybe train it by randomly varying things and hitting a button with mood
- [ ] A book about a dystopian future where instead of creating solutions around medical care, they just used cloning.
- [ ] Alternative school system
- [ ] The relationship between seemingly redundant verbose communication that is more flexible, and more efficient abstractions that are tightly coupled to specific circumstances
- [ ] Call into religious show to explain ego id architipal conversational meditation
- [ ] Dev teching business
- [ ] AR business Ideas
- [ ] AI business ideas
- [ ] Bucket list creation things
- [ ] Dream lining!
- [ ] Fear setting
- [ ] How americans being fond of british accents might be related to hundreds of years of slavery
- [ ] What if speach to text was to also run a model for mood for tone and use that to not only type better but to give better suggestions as well.
- [ ] Sell Gary Vee  Software that summarizes comments on social media
- [ ] Software that aggregates personality test results
- [ ] Preventative governneces vs punishing on effect
- [ ] Create a youtube channel where all you do is improve music and videos/movies
- [ ] Personality tests for dev teams gauging them on axis of readability/ extendability etc ie Clean Code
- [ ] A story that instead of humanity surviving evolutionarily from good sight, e survived more due to good sounds, so as humans we built technologies that work really well with loading our sound senses with excess data from using sound (still has eyes but not as prominent) sonar personal devices to act as VR with your ears, types of music, human crimes with noises, increased memory due to less memory requirements of sound rather than video, screens without sight etc
- [ ] AR Smart Watches(like pebble) using an qr code or some other form of tracking and other AR accessories
- [ ] An app called atlatl that involves sending things back and forth
- [x] - Religion might just be a group of people using evolutionary group/tribe war dynamics to experience mystical experiences similar to sensory deprivation/ psychedelic drugs(fake abstraction building)
- [x] - Religion might be an optimization algo of the brain
- [x] - AR manifesto, Abstract concept and benefits(future like phones), engineering problems and how to solve them(env tracking/ obj recognition, displays, head tracking, controls, form factor), auxiliary tech after(tactile simulation, body integration, AR/VR transfer, voice control/isolation mask)
- [x] - How children should be learning the process of engineering
- [x] - distributed operating system
- [x] - small unintelligent things combining to create intelegence
- [x] Just get one person to buy one thing
- [x] Startup vs Job
- [x] Using Dimension reduction to create approximation of compressible patterns using compression algorithms then using those compressed approximation to predict the future, using by looking at permutations of the approximation before consistent patterns and storing them for future use
- [x] Creating a game based on the heat death of the universe
- [x] Ego Id architipal conversational meditation
- [x] Get proximity/distance in 4+ dimentions using different weights on each dimention calculated by predictive clustering
- [x] Things i think it takes to learn to be a developer
- [x] Reversable dimention reduction for AI
- [x] - Build a website that aggregates data from different cities and ranks cities based on jobs pay to cost of living.
