;; Token Economics
;; Contains the Bitcoin-style tokenomics constants

;; Bitcoin-style tokenomics constants
(define-constant TOTAL-SUPPLY u21000000000) ;; 21 billion tokens
(define-constant INITIAL-BLOCK-REWARD u5000) ;; 5,000 tokens per block
(define-constant HALVING-INTERVAL u210000) ;; Halving every 210,000 blocks
(define-constant DEX-ALLOCATION-PERCENTAGE u30) ;; 30% to DEX
(define-constant TEAM-ALLOCATION-PERCENTAGE u15) ;; 15% to team
(define-constant DAO-ALLOCATION-PERCENTAGE u55) ;; 55% to DAO/community

;; Read-only functions to access core constants
(define-read-only (get-total-supply)
    (ok TOTAL-SUPPLY)
)

(define-read-only (get-block-reward)
    (ok INITIAL-BLOCK-REWARD)
)

(define-read-only (get-halving-interval)
    (ok HALVING-INTERVAL)
)

(define-read-only (get-allocation-percentages)
    (ok {
        dex: DEX-ALLOCATION-PERCENTAGE,
        team: TEAM-ALLOCATION-PERCENTAGE,
        dao: DAO-ALLOCATION-PERCENTAGE
    })
)