M1 2 1 0 n 10e-6 0.35e-6 2
M2 2 1 3 p 30e-6 0.35e-6 1

V1 1 0 DC 1.54
VDD 3 0 DC 3
R1 2 0 1e10

.MODEL 1 VT -0.75 MU 5e-2 COX 0.3e-4 LAMBDA 0.05 CJ0 4.0e-14
.MODEL 2 VT 0.83 MU 1.5e-1 COX 0.3e-4 LAMBDA 0.05 CJ0 4.0e-14
