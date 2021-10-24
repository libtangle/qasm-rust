// QFT and measure, version 2
OPENQASM 2.0;

qubit q[4];
creg c[4];

reset q;
h q;
barrier q;
h q[0];
measure q[0] -> c[0];
if (c == 1) rz(pi / 2) q[1];
h q[1];
measure q[1] -> c[1];
if(c==1) rz(pi / 4) q[2];
if(c==1) rz(pi / 2) q[2];
h q[2];
measure q[2] -> c[2];
if(c == 1) rz(pi / 8) q[3];
if(c == 1) rz(pi / 4) q[3];
if(c == 1) rz(pi / 2) q[3];
h q[3];
measure q[3] -> c[3];