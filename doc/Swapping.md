A swap is an atomic batch-transfer of UTXOs between `n` participants. Either all `n` transfers complete or none of them do. A swap is initiated by a coordinator providing a `batch_id` and `n` signatures - one from each participant. The signatures sign for a UTXO to be involved in a swap with a particular `batch_id` and to occur within some time frame. 

A Coordinator is responsible for finding and coordinating a group of particiapnts who wish to swap. Once the desired number of participants is reached the coordinator (blindly) matches senders and receivers together for each individual transfer. The swap can then commence.


## API

The transfer protocol is implemented as two functions. The sender calls `transfer_sender()` to performs their half of the protocol and the receiver calls `transger_receiver()` to perform their half and thus complete the transfer.

For the StateChain Entity (SCE) side of performing swaps we have a `transfer_batch_init()` function which takes a list of:

- state chain IDs
- StateChain signature of batch transfer - a sig for this state chain representing desire to be included in a particular batch and at a particular time

for each participant in the Swap Pool. 

After initialising the swap, the SCE waits for each `transfer_sender()` involved in the swap and for the corresponding `transfer_receiver()` calls to complete. If all complete within some lifetime then all transfers are finalized. If one or more transfers are not complete after some amount of time the batch transfer is cancelled and none of the transfers are finalized. 


## DoS Protection

To prevent spam attacks there must be a way for the SCE to identify which participant to punish in the event of a failed swap. 

### Punish both participants involved in a failed transfer

Any failed transfers can be identified and punished easily. To identify the Receiver of a filed transfer each participant must commit to their input UTXO when carrying out `transfer_receiver()`, which can later be requested for reveal by the SCE if some particiaptns do not complete their transfers in time. SCE can then link the completed transfers with their original owners and punish those whose commitments are not present. The swap pool can then restart with a new owner-to-UTXO swap mapping.

This means that a participant is punished if either of the 2 transfers they are involved in do not succeed. 

The following failure circumstances can occur for an individual transfer:

1) Sender fails to complete `transfer_sender`
2) Sender fails to send the transfer informtion to Receiver
3) Receiver fails to complete `transfer_receiver`

It is obvious to the server who to punish if 1) occurs. They can see which UTXOs involvde in the swap did not carry out `transfer_sender`.

However in the cases of 2) and 3) the server does not know whether Sender failed to send the transfer message to Receiver or whether Receiver failed to complete their end of the protocol. Therefore both parties must be punished here which is a bit clumsy.

### Punish only the misbehaving/unresponsive participant

Another option is to have transfer messages passed via the swap pool Coordinator and have punishments carried out by the Coordinator since they would be able to tell the difference between a 2) and a 3) failure. However this adds quite a bit of complexity:

A failed batch transfer still costs the SCE so a fidelity bond would have to be provided by the coordinator for each run to prevent DoSing the SCE. Now the coordinator must fund the fidelity bond somehow and so would have to have their own fidelity bond system for each participant in the batch-transfer (A coordinator would not be willing to lose a fidelity bond of their own coins if it is a participant's fault that the swap failed). 

To verify that a valid message has been sent from Sender to Receiver some kind of `transfer_receiver` failure proof could be generated by the SCE after attempting to perform the function with an invliad message passed from Sender. Inability for the Receiver to provide such a proof would give the Coordinator good reason to punish the Receiver.

This also involves the coordinator far more in the protocol. Their job has gone from a simple participant matcher to now storing bonds, passing messages around and verifying proofs. 