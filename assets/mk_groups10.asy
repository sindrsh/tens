import inh_gr;

void db(int I=1, int J=1, pen p=blue, pen p1=p, pair sh = (0,0)){
	path b = box((0,0),(1,1));
	for (int i = 1; i <= I; ++i){
		for (int j = 1; j <= J; ++j){
			filldraw(shift(sh+(i-1,j-1))*b, p, fillpen=p1); 
			} 
		}
}

for (int i=1; i<=9;++i){
	db(I=10,J=1, p=black, p1=heavygreen, sh=(0,-i*d));
	string s_i = (string) i;
	shipout("fig"+s_i+"0");	
}

