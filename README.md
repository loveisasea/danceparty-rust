This is a rust implementation for the following programming exercise.

===============================================================================================================
任务：
 
有n 个领舞和m个follower在一个舞蹈学校。 每一个舞者有一个记录卡写有他和谁跳了哪只舞。分别有以下8种形式：
1.  Waltz
2.  Tango
3.  Foxtrot
4.  Quickstep
5.  Rumba
6.  Samba
7.  Cha Cha
8.  Jive
设想每个人都想跳每只舞，但他们不能和同一个舞伴跳超过两支舞。领舞邀请一个follower选择他们想跳哪只舞, 与此同时不能邀请其他人，直到他知道他正在邀请的人是拒绝还是接受他。 如果接受，他们开始邀请其他的人完成他们记录卡上的其他舞。Follower等待邀请，并接受领舞的邀请（可能情况1:如果他们没有和任何领舞跳过那种舞，即可以接受。情况2:他们如果已经接受和该领舞跳另外两种舞了，则不能接受）
 
l   每个领舞如果在以上8种形式的舞都找到了舞伴，即停止。
l   当所有领舞停止寻找时，配对过程结束，最终结果反馈出来。（不必要完美的配对，每个领舞都刚好能在每个舞种上找到舞伴，但当程序停止时，将不再有可能的变动）
 
规则：
l   有n 个领舞和m个follower
l   每个人只有一次机会和另外一个人跳同一种舞（例如：你不能跳两遍Waltz）
l   领舞邀请follower 的具体实现方法不限（例如可能的有：随机选择，按次序选择，从他们自己的号码相同的follower开始依次选等等不限）
l   领舞必须等待正在邀请的人回复以后才能邀请下一个人。注意：领舞可能在一下这种情况下邀请失败 e.g.,如果你尝试将一个值放进一个 buffer 但被告知没放入, 你可以邀请其他人来跳舞. (Some libraries will offer non-blocking put functions that return a boolean: true for "yes, you successfully put the value in the buffer" or false for "no, there was no room in the buffer, and I didn't wait around for things to change". If you are only using functionality that definitely blocks when attempting to put into a full buffer, you may indeed have all your leaders queued up, waiting to ask some particular follower for a dance!).
l   如果领舞没有人可以邀请了，此时领舞停止。
l   follower判断是否接受跳舞的情况（可能情况1:如果他们没有跳过那种舞，即可以接受。情况2:他们如果已经接受和该领舞跳另外两种舞了，则不能接受）
 
 
 
 
要求：
l   每一个人是一个独立的thread
l   另一个“main” thread 应该用来打印最后的结果
l   每一个人有一个资源库（类似于一个buffer）可以用来接收别人的消息，并且只有本人可以阅读自己的资源库，且每次只能处理一条消息(these communication resources must all be collectively accessible at the same time as each other, but individually accessible by only one person at a time.)
l   由命令行输入需要的两个整型变量，先n后m
l   领舞会被编号1到n, follower 会被编号1-m
l   输出结果如下（“——”表示没有配到舞伴）
 
Leader 1:
Waltz      with 3
Tango      with 5
Foxtrot    with 2
Quickstep  with 1
Rumba      ------
Samba      with 2
Cha Cha    with 3
Jive       ------
 
Leader 2:
Waltz      with 2
Tango      with 4
...
 
不用使每一个领舞在每一个舞种上找到了舞伴（完美配对），有时可能会产生配不到的情况。而且n 和m也可能是两个不同的数
 

 

===============================================================================================================

Task

You and your partner will implement the following concurrent program in an imperative language (either C pthreads or Java threads, or please pre-approve your choice). Next, you'll implement the program in Haskell using concurrent mechanisms. Lastly, you will write up the experience with any relevant details, things you tried that was surprising or difficult or informative, and any build-instructions needed to run your code.

There are N leaders and M followers at the Socrates School of Dance and Deep Thought. Each dancer has a dance card that should be filled out by listing the person with whom they will dance that song. It has the following eight entries on it:

9.  Waltz
10.              Tango
11.              Foxtrot
12.              Quickstep
13.              Rumba
14.              Samba
15.              Cha Cha
16.              Jive
Everybody wants to dance each of the dances, but they cannot dance more than two dances with the same partner - it would be impolite. Leaders invite any follower of their choosing to dance a specific dance, and cannot ask anyone else to dance until they are either accepted or rejected. If accepted, they start asking anyone for other available dances on their own dance card. Followers wait for invitations, and then accept a dance if they aren't already dancing that one (or haven't already agreed to two dances with that leader).

•   A leader with a full dance card also stops searching for partners.
•   When all leaders have stopped searching for partners, the matching process is over and results should be reported. It is not necessary to achieve perfect matching, but there must be no more possible moves when your program stops.
Rules

•   There will be N leaders and M followers.
•   One dance of each style will play, for a total of eight songs; this means that every individual has one chance at dancing with someone during each song (e.g., you can't dance two+ waltzes).
•   leaders employ any strategy you can think of to ask any follower to dance with them for any particular dance.
◦                             some possibilities: random selection, sequential, beginning with their own number (it's okay for them to know they are leader #5 of 8, for example), or any alternative of your own devising.
•   leaders must wait on a response before they can ask anyone else. Note, though, that implementation-wise, you may find that a leader attempted to ask a follower but the attempt failed in some sense - e.g., if you tried to put a value into a buffer but were told it didn't happen, then you are free to instead go ask someone else to dance. (Some libraries will offer non-blocking put functions that return a boolean: true for "yes, you successfully put the value in the buffer" or false for "no, there was no room in the buffer, and I didn't wait around for things to change". If you are only using functionality that definitely blocks when attempting to put into a full buffer, you may indeed have all your leaders queued up, waiting to ask some particular follower for a dance!).
•   when there is nobody else a leader can ask to dance for any dances, the leader is done attempting to fill out their dance card.
•   followers wait for invitations, and then respond "yes" if they don't yet have a dance partner for the dance, and if they haven't already agreed to dance with this person for two songs. If you want to make the followers' logic more complicated than "yes unless it has to be no", you're welcome to do so, but it will make the project harder.
Requirements

•   each person must be an individual thread.
•   another "main" thread might be necessary to announce (print) the results at the end.
•   everybody communicates with each other through some notion of a mailbox: each person has a dedicated resource (like a buffer) that anyone can attempt to send a message to, and only that person can read from. Only one message can be pending at a time, though.
◦                             these communication resources must all be collectively accessible at the same time as each other, butindividually accessible by only one person at a time.
•   two command line arguments, integers representing N and then M, must be provided on the command line.
•   Leaders will be numbered from 1 to N. Followers will be numbered from 1 toM.
•   results will be presented in this exact fashion: "Leader X:" on the first line, followed by the eight dances and the follower (or "——" for unassigned) listed afterwards in a second column. By listing all leaders' dance cards, we have enough information.
Leader 1:
Waltz      with 3
Tango      with 5
Foxtrot    with 2
Quickstep  with 1
Rumba      ------
Samba      with 2
Cha Cha    with 3
Jive       ------
 
Leader 2:
Waltz      with 2
Tango      with 4
...
You do not have to find a "perfect" matching for all participants; some may end up with no available dances, even though others are still not dancing all dances. Perhaps the wrong dance is available, or they've already agreed to two other dances with each other, or N andM may be different values.

The Writeup

Because we are double-implementing a task, it is not supposed to be a particularly large task. As a last step, you will write up a brief summary of what you experienced (as a part of the README.txt file). There's no page limit, just make sure you've focused on each of the questions below. Perhaps a page and a half or two would be sufficient. Some questions that need to be answered here are:

•   For your programming task, what were the challenges that you faced? Where was there competition for resources, and where was there a need for cooperation?
•   In your imperative implementation, what aspects of the task were straightforward, and which ones felt laborious?
•   What kinds of bugs did you run into for the imperative implementation –deadlock? How did you attempt to inspect what was going wrong with the code? (Did you use any debuggers or anything? You weren't required to do so, I'm just curious how your experience went).
•   In your Haskell implementation, what aspects of the task were straightforward, and which ones felt laborious?
•   Again, what kinds of bugs arose during development, and how did you handle them?
•   Lastly, what piece of advice do you wish you had received at the start of the assignment?
Grading

Your score will be based on calculating consistent outcomes; for instance, no person can dance with multiple other people during the same dance. Also, if a lead and follow were both idle during a song they could have danced together, it isn't a consistent result. We will also be manually checking for the following aspects to your solution:

•   actual use of separate threads where appropriate.
•   no "global lock" : While one person is dealing with a request, all uninvolved people can be busily working on their own dance cards.
•   your code doesn't devolve to some predictable ordering. If you find some pattern, for instance if leader 1 always dances with follows 1, 1, 2, 2, 3, 3, 4, 4, and leader 2 always dances with partners 2, 2, 3, 3, 4, 4, 5, 5, then we need to shift something to make sure the outcomes can vary from run to run.
Scoring Breakdown

•   60%: consistent outcomes (split evenly between the two implementations)
•   25%: adhering to the
•   15%: completed writeup with reasonable content.
 
 
===============================================================================================================

